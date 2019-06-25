use clap::arg_enum;
use io::{BufRead, Write};
use std::{convert, error, fmt, io, process};
use unicode_normalization::UnicodeNormalization;

fn main() {
    process::exit(match run() {
        Err(Error::Clap(e)) => {
            eprint!("{}", e);
            2
        }
        Err(e) => {
            eprintln!("{}", e);
            1
        }
        Ok(_) => 0,
    });
}

fn run() -> Result<(), Error> {
    let final_newline_default = true.to_string();
    let normalization_default = Normalization::Nfc.to_string();
    const OUTPUT_SEPARATOR_DEFAULT: &str = " ";
    let matches = clap::App::new(clap::crate_name!())
        .setting(clap::AppSettings::StrictUtf8)
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .after_help("The only supported input and output encoding is UTF-8.")
        .arg(
            clap::Arg::with_name("input")
                .help("Input strings to normalize; if not present, input is read from stdin.")
                .multiple(true)
                .value_name("INPUT"),
        )
        .arg(
            clap::Arg::with_name("final-newline")
                .help("Whether to end the output with a newline.")
                .long("final-newline")
                .case_insensitive(false)
                .default_value(&final_newline_default)
                .possible_values(&[&true.to_string(), &false.to_string()])
                .value_name("BOOLEAN"),
        )
        .arg(
            clap::Arg::with_name("normalization")
                .help("Unicode Normalization type to use.")
                .short("n")
                .long("normalization")
                .case_insensitive(true)
                .default_value(&normalization_default)
                .possible_values(&Normalization::variants())
                .value_name("NORMALIZATION"),
        )
        .arg(
            clap::Arg::with_name("output-separator")
                .help(&format!(
                    "Output separator for command-line input arguments. [default: {:?}]",
                    OUTPUT_SEPARATOR_DEFAULT
                ))
                .long("output-sep")
                .default_value(OUTPUT_SEPARATOR_DEFAULT)
                .hide_default_value(true)
                .value_name("SEPARATOR"),
        )
        .get_matches_safe()?;

    let final_newline = clap::value_t!(matches.value_of("final-newline"), bool)?;
    let normalization = clap::value_t!(matches.value_of("normalization"), Normalization)?;
    let mut out = io::stdout();

    match matches.values_of("input") {
        Some(values) => {
            let in_ = values.map(Result::<_, convert::Infallible>::Ok);
            let separator = matches.value_of("output-separator").unwrap();
            print_normalized(normalization, separator, in_, &mut out)?;
        }
        None => {
            let stdin = io::stdin();
            let in_ = stdin.lock().lines();
            let separator = "\n";
            print_normalized(normalization, separator, in_, &mut out)?;
        }
    };

    if final_newline {
        out.write_all("\n".as_bytes())?;
    }

    Ok(())
}

fn print_normalized<E, I, S, W>(
    normalization: Normalization,
    separator: &str,
    in_: I,
    out: W,
) -> Result<(), Error>
where
    Error: From<E>,
    I: Iterator<Item = Result<S, E>>,
    S: AsRef<str>,
    W: io::Write,
{
    use Normalization::*;
    match normalization {
        Nfc => print_normalized_with(|s| s.nfc().collect(), separator, in_, out),
        Nfd => print_normalized_with(|s| s.nfd().collect(), separator, in_, out),
        Nfkc => print_normalized_with(|s| s.nfkc().collect(), separator, in_, out),
        Nfkd => print_normalized_with(|s| s.nfkd().collect(), separator, in_, out),
    }
}

fn print_normalized_with<E, I, N, S, W>(
    normalize: N,
    separator: &str,
    mut in_: I,
    mut out: W,
) -> Result<(), Error>
where
    Error: From<E>,
    I: Iterator<Item = Result<S, E>>,
    N: Fn(&str) -> String,
    S: AsRef<str>,
    W: io::Write,
{
    let write_normalized = |out: &mut W, s: Result<S, E>| -> Result<(), Error> {
        let normalized = normalize(s?.as_ref());
        out.write_all(normalized.as_bytes())?;
        Ok(())
    };

    if let Some(s) = in_.next() {
        write_normalized(&mut out, s)?;
    }
    for s in in_ {
        out.write_all(separator.as_bytes())?;
        write_normalized(&mut out, s)?;
    }
    Ok(())
}

arg_enum! {
    #[derive(Debug)]
    enum Normalization {
        Nfc,
        Nfd,
        Nfkc,
        Nfkd,
    }
}

#[derive(Debug)]
enum Error {
    Clap(clap::Error),
    Io(io::Error),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Clap(e) => e.fmt(f),
            Error::Io(e) => e.fmt(f),
        }
    }
}

impl From<clap::Error> for Error {
    fn from(e: clap::Error) -> Self {
        Error::Clap(e)
    }
}

impl From<convert::Infallible> for Error {
    fn from(_: convert::Infallible) -> Self {
        unreachable!()
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
