#[macro_use]
extern crate clap;
extern crate num_cpus;
#[macro_use]
extern crate quick_error;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::{cmp, fmt, fs, io, path, process, sync, thread};
use std::fmt::Write as WriteFmt;
use std::io::prelude::*;
use strum::IntoEnumIterator;

type Res<T> = Result<T, Error>;

fn main() {
    process::exit(match run() {
        Ok(status) => status,
        Err(Error::Clap(e)) => match e.kind {
            clap::ErrorKind::HelpDisplayed |
            clap::ErrorKind::VersionDisplayed => {
                print!("{}", e);
                ExitStatus::Same
            },
            _ => {
                eprint!("{}", e);
                ExitStatus::Err
            },
        },
        Err(e) => {
            eprintln!("{}", e);
            ExitStatus::Err
        },
    }.into());
}

fn run() -> Res<ExitStatus> {
    let threads_max_default = num_cpus::get().to_string();

    let args = clap::App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .after_help(format!(
            "Print whether the files are the same, and indicate it via the \
             exit status as well.\n\
             \n\
             A regular file is considered the same as another one if they are \
             the same size and have the exact same contents.\n\
             \n\
             Other file types are not supported.
             \n\
             Exit status:\n\
             {}",
            {
                let mut text = String::new();
                for status in ExitStatus::iter() {
                    write!(text, " {:3} — {}\n", i32::from(status), status)?;
                }
                text.pop();
                text
            }).as_str())
        .arg(clap::Arg::with_name("PATH")
             .help("The files to compare.")
             .min_values(2)
             .required(true))
        .arg(clap::Arg::with_name("buffer-size")
             .short("b")
             .long("buffer-size")
             .help("The size of the buffers used for reading files.")
             .default_value("4096")
             .takes_value(true)
             .validator(|buffer_size| {
                 let buffer_size: usize = buffer_size.parse()
                     .map_err(|e| format!("{}", e))?;
                 if buffer_size <= 0 {
                     Err(format!("must be greater than 0"))
                 } else {
                     Ok(())
                 }
             })
             .value_name("BYTES"))
        .arg(clap::Arg::with_name("quiet")
             .short("q")
             .long("quiet")
             .help("\
                 Print no output except error messages; the status code is the \
                 only output."))
        .arg(clap::Arg::with_name("threads-max")
             .short("t")
             .long("threads-max")
             .help(&format!("\
                 Maximum number of threads to use. [default: {} (number of \
                 logical cores in the system)]", threads_max_default))
             .default_value(&threads_max_default)
             .hide_default_value(true)
             .takes_value(true)
             .validator(|threads_max| {
                 let threads_max: usize = threads_max.parse()
                     .map_err(|e| format!("{}", e))?;
                 if threads_max <= 0 {
                     Err(format!("must be greater than 0"))
                 } else {
                     Ok(())
                 }
             }))
        .get_matches_safe()?;

    let path_bufs = args.values_of("PATH").expect("no PATH")
        .map(path::PathBuf::from)
        .collect::<Vec<_>>();
    let paths = path_bufs.iter()
        .map(|p| p.as_path())
        .collect::<Vec<_>>();
    let buffer_size = value_t!(args.value_of("buffer-size"), usize)?;
    let print_comparison = !args.is_present("quiet");
    let threads_max = value_t!(args.value_of("threads-max"), usize)?;

    let comparison = compare_all(paths.as_slice(), buffer_size, threads_max)?;
    if print_comparison {
        println!("{}", comparison);
    }
    Ok(comparison.into())
}

fn compare_all(paths: &[&path::Path], buffer_size: usize, threads_max: usize) ->
    Res<Comparison>
{
    let (first, rest) = paths.split_at(1);
    let first = *first.first().expect("no first element in first");
    let metadata_first = fs::metadata(first).map_err(|e| (e, first))?;
    if !metadata_first.file_type().is_file() {
        return Err(Error::FileTypeUnsupported(
            first.into(), metadata_first.file_type()).into())
    }

    for &path in rest {
        let metadata = fs::metadata(path).map_err(|e| (e, path))?;
        if !metadata.file_type().is_file() {
            return Err(Error::FileTypeUnsupported(
                path.into(), metadata.file_type()).into());
        }
        if metadata.len() != metadata_first.len() {
            return Ok(Comparison::DifferentSize {
                left: first.into(), len_left: metadata_first.len(),
                right: path.into(), len_right: metadata.len(),
            });
        }
    }

    let mut comparer = Comparer::new(&first, metadata_first.len(),
                                     threads_max, buffer_size)?;
    for path in rest {
        match comparer.compare(&path)? {
            Comparison::Same => (),
            comparison => return Ok(comparison),
        };
    }
    Ok(Comparison::Same)
}

#[derive(Clone)]
struct Comparer {
    to_path: path::PathBuf,
    to_len: u64,
    threads: usize,
    buffer_size: usize,
}

impl Comparer {
    fn new(to: &path::Path, to_len: u64, threads_max: usize,
           buffer_size: usize) -> Res<Self> {
        let blocks = to_len as usize / buffer_size;
        const THREAD_BLOCKS_MIN: usize = 500;
        let threads = blocks / THREAD_BLOCKS_MIN;
        let threads = cmp::max(cmp::min(threads, 1), threads_max);
        Ok(Self {
            to_path: to.into(),
            to_len,
            threads,
            buffer_size,
        })
    }

    fn compare(&mut self, file_path: &path::Path) -> Res<Comparison> {
        if self.threads == 1 {
            // 1 thread: do it on the current thread; do not spawn any new ones
            return self.compare_segment(file_path, 0, self.to_len);
        }

        // multiple threads: spawn each one and wait for the first non-Same
        // comparison; if all comparisons are Same, the enitre compare is Same
        let blocks = self.to_len as usize / self.buffer_size;
        let blocks_leftover = self.to_len as usize % self.buffer_size;
        let blocks_thread = blocks / self.threads;
        let threads_large = blocks % self.threads;

        let (comparison_send, comparison_recv) = sync::mpsc::channel();
        for i in 0..self.threads {
            let comparison_send = comparison_send.clone();
            let file_path: path::PathBuf = file_path.into();
            let comparer = self.clone();
            thread::spawn(move || {
                let beg_blocks = i * blocks_thread + cmp::min(i, threads_large);
                let beg = (beg_blocks * comparer.buffer_size) as u64;
                let len_blocks = blocks_thread +
                    if i < threads_large { 1 } else { 0 };
                let len = len_blocks * comparer.buffer_size +
                    if i < comparer.threads - 1 { 0 } else { blocks_leftover };
                let end = beg + len as u64;
                comparison_send.send(
                    comparer.compare_segment(&file_path, beg, end))
            });
        }

        for _ in 0..self.threads {
            match comparison_recv.recv().unwrap()? {
                Comparison::Same => (),
                comparison => return Ok(comparison),
            }
        }
        Ok(Comparison::Same)
    }

    fn compare_segment(&self, file_path: &path::Path, beg: u64, end: u64) ->
        Res<Comparison>
    {
        let to = fs::File::open(&self.to_path).map_err(|e| (e, &self.to_path))?;
        let mut to = io::BufReader::with_capacity(self.buffer_size, to);
        let file = fs::File::open(file_path).map_err(|e| (e, file_path))?;
        let mut file = io::BufReader::with_capacity(self.buffer_size, file);

        let beg_pos = io::SeekFrom::Start(beg);
        to.seek(beg_pos)
            .expect(&format!("cannot seek to {:?}: {:?}",
                             beg_pos, self.to_path));
        file.seek(beg_pos)
            .expect(&format!("cannot seek to {:?}: {:?}", beg_pos, file_path));

        let mut pos = beg;
        loop {
            if pos >= end {
                // checked up to or past position `to`
                break Ok(Comparison::Same);
            }
            let len = {
                //let to_path = self.to_path.clone();
                let buf_to = to.fill_buf().map_err(|e| (e, &self.to_path))?;
                let buf_file = file.fill_buf().map_err(|e| (e, file_path))?;
                if buf_to.is_empty() && buf_file.is_empty() {
                    // EOF at same position
                    break Ok(Comparison::Same);
                }
                if buf_to.len() != buf_file.len() {
                    // EOF at different positions
                    return Ok(Comparison::DifferentSize {
                        left: self.to_path.clone(),
                        len_left: self.to_path.metadata()
                            .map_err(|e| (e, &self.to_path))?
                            .len(),
                        right: file_path.into(),
                        len_right: file_path.metadata()
                            .map_err(|e| (e, file_path))?
                            .len(),
                    });
                }
                if buf_to != buf_file {
                    return Ok(Comparison::DifferentContents {
                        left: self.to_path.clone(),
                        right: file_path.into(),
                    });
                }
                buf_to.len()
            };
            to.consume(len);
            file.consume(len);
            pos += len as u64;
        }
    }
}

enum Comparison {
    Same,
    DifferentSize { left: path::PathBuf, len_left: u64,
                    right: path::PathBuf, len_right: u64 },
    DifferentContents { left: path::PathBuf, right: path::PathBuf },
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Comparison::Same => write!(f, "files are the same"),
            Comparison::DifferentSize { left, len_left, right, len_right } =>
                write!(f, "files have different sizes: \
                           {:?}: {} B -- {:?}: {} B",
                       left, len_left, right, len_right),
            Comparison::DifferentContents { left, right } =>
                write!(f, "files have different contents: {:?} -- {:?}",
                       left, right),
        }
    }
}

impl From<Comparison> for ExitStatus {
    fn from(comparison: Comparison) -> Self {
        match comparison {
            Comparison::Same => ExitStatus::Same,
            Comparison::DifferentSize { .. } |
            Comparison::DifferentContents { .. } =>
                ExitStatus::Different,
        }
    }
}

#[derive(Clone, Copy, EnumIter)]
enum ExitStatus {
    Same,
    Different,
    Err,
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            ExitStatus::Same => "files have the same contents",
            ExitStatus::Different => "files have different contents",
            ExitStatus::Err => "an error occured",
        })
    }
}

impl From<ExitStatus> for i32 {
    fn from(exit_status: ExitStatus) -> Self {
        match exit_status {
            ExitStatus::Same => 0,
            ExitStatus::Different => 1,
            ExitStatus::Err => i8::max_value().into(),
        }
    }
}

quick_error!{
    #[derive(Debug)]
    enum Error {
        Clap(e: clap::Error) {
            cause(e) description(e.description()) display("{}", e) from()
        }
        FileAccessDenied(path: path::PathBuf, e: io::Error) {
            cause(e)
            description("access to file denied")
            display("access to file denied: {}: {:?}", e, path)
        }
        FileNotFound(path: path::PathBuf) {
            description("file not found")
            display("file not found: {:?}", path)
        }
        FileTypeUnsupported(path: path::PathBuf, file_type: fs::FileType) {
            description("file type unsupported")
            display("file type {} unsupported: {:?}",
                    file_type_to_string(file_type), path)
        }
        Fmt(e: fmt::Error) {
            cause(e) description(e.description()) display("{}", e) from()
        }
        Io(e: io::Error) {
            cause(e) description(e.description()) display("{}", e)
        }
    }
}

fn file_type_to_string(file_type: &fs::FileType) -> String {
    if file_type.is_dir() {
        format!("directory")
    } else if file_type.is_file() {
        format!("regular file")
    } else if file_type.is_symlink() {
        format!("symbolic link")
    } else {
        format!("unknown ({:?})", file_type)
    }
}

impl<P: AsRef<path::Path>> From<(io::Error, P)> for Error {
    fn from(e: (io::Error, P)) -> Self {
        let (e, path) = e;
        match e.kind() {
            io::ErrorKind::NotFound =>
                Error::FileNotFound(path.as_ref().into()),
            io::ErrorKind::PermissionDenied =>
                Error::FileAccessDenied(path.as_ref().into(), e),
            _ => Error::Io(e),
        }
    }
}
