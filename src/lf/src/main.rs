/**
 * Call `ls` with `-l` if its output fits on screen, with `-C` otherwise.
 */

extern crate term_size;

use std::env;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::process;

type Result<T> = std::result::Result<T, String>;

fn prompt_fish() -> Result<String> {
    let mut command = process::Command::new("fish");
    command.arg("-c")
        .arg("fish_prompt")
        .stderr(process::Stdio::inherit());

    let output = command.output()
        .map_err(|e| format!("{:?} failed: {}", command, e))?;
    if output.status.success() {
        String::from_utf8(output.stdout)
            .map_err(|e| format!("{:?} output: {}", command, e))
    } else {
        Err(format!("{:?} status: {:?}", command, output.status.code()))
    }
}

fn prompt_bash() -> Result<String> {
    const PROMPT_VAR: &str = "PS1";
    env::var(PROMPT_VAR).map_err(|e| format!("{}: {}", PROMPT_VAR, e))
}

fn prompt_lines() -> Result<usize> {
    const SHELL_VAR: &str = "SHELL";
    let shell = match env::var(SHELL_VAR) {
        Err(_) => None,
        Ok(s) => match Path::new(&s).file_name() {
            None => None,
            Some(s) => {
                Some(s.to_os_string().into_string()
                     .map_err(|s| format!("{} filename: {:?}", SHELL_VAR, s))?)
            },
        },
    };
    let prompt = match shell {
        None => None,
        Some(s) => Some(match s.as_str() {
            "fish" => prompt_fish()?,
            _ => prompt_bash()?,
        }),
    };
    Ok(prompt.map_or(0, |s| s.lines().count()))
}

fn output_ls<I, S>(args: I) -> Result<Vec<u8>>
    where I: IntoIterator<Item=S>, S: AsRef<std::ffi::OsStr>
{
    let mut command = process::Command::new("ls");
    command.args(args)
        .stderr(process::Stdio::inherit());

    command.output()
        .map(|output| output.stdout)
        .map_err(|e| format!("{:?} failed: {}", command, e))
}

fn run() -> Result<()> {
    let size = term_size::dimensions_stdout();

    let args = {
        let mut args: Vec<String> = env::args().skip(1).collect();
        // explicitly specify width, since ls has no way of querying it when
        // it's a child process of this one
        if let Some((width, _)) = size {
            args.push(String::from("--width"));
            args.push(width.to_string());
        }
        args
    };

    let long = {
        let mut args = args.clone();
        args.push(String::from("-l"));
        output_ls(args)?
    };

    let output = if let Some((_, height)) = size {
        let lines_max = height - prompt_lines()?;
        let long_str = String::from_utf8(long.clone())
            .map_err(|e| e.to_string())?;
        if long_str.lines().count() <= lines_max {
            long
        } else {
            let mut args = args.clone();
            // `-C` forces ls to arrange output in columns, even when it detects
            // the output as not a tty
            args.push(String::from("-C"));
            output_ls(args)?
        }
    } else {
        long
    };

    io::stdout().write(&output)
        .map_err(|e| format!("failed write to stdout: {}", e))?;
    Ok(())
}

fn main() {
    process::exit(match run() {
        Err(e) => {
            eprintln!("error: {}", e);
            1
        },
        Ok(_) => 0,
    });
}
