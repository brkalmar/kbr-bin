#[macro_use]
extern crate clap;
extern crate iso_9_convert as lib;

use std::error;
use std::io::prelude::*;
use std::io;

type Res<T> = Result<T, Box<error::Error>>;

fn main() {
    std::process::exit(match run() {
        Err(e) => {
            eprintln!("{}", e);
            1
        },
        Ok(status) => status,
    });
}

fn run() -> Res<i32> {
    let hard_soft_sign_default = lib::HardSoftSign::default().to_string();
    let hook_to_left_default = lib::HookToLeft::default().to_string();
    let matches = clap::App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(clap::Arg::with_name("TO")
             .help("which script to convert to")
             .case_insensitive(true)
             .possible_values(&Script::variants())
             .required(true))
        .arg(clap::Arg::with_name("TEXT")
             .help("the text to convert; if absent, stdin is used")
             .multiple(true))
        .arg(clap::Arg::with_name("hard-soft-sign")
             .long("hard-soft-sign")
             .help("whether to use capital (Ь, Ъ) or small (ь, ъ) hard and \
                    soft signs for Cyrillic")
             .case_insensitive(true)
             .default_value(&hard_soft_sign_default)
             .possible_values(&lib::HardSoftSign::variants()))
        .arg(clap::Arg::with_name("hook-to-left")
             .long("hook-to-left")
             .help("whether to use cedilla (e.g. ş) or comma below (e.g. ș) \
                    for certain characters in Latin")
             .case_insensitive(true)
             .default_value(&hook_to_left_default)
             .possible_values(&lib::HookToLeft::variants()))
        .get_matches_safe()?;

    let stdin;
    let mut iter_stdin;
    let mut iter_values;
    let (input, sep): (&mut Iterator<Item = Res<String>>, &str) =
        match matches.values_of("TEXT") {
            Some(values) => {
                iter_values = values.map(|s| Ok(s.to_string()));
                (&mut iter_values, " ")
            },
            None => {
                stdin = io::stdin();
                iter_stdin = stdin.lock().lines().map(|l| {
                    l.map_err(|e| Box::from(e))
                });
                (&mut iter_stdin, "\n")
            },
        };

    let converter = match value_t!(matches.value_of("TO"), Script)? {
        Script::Cyrillic => lib::Converter::to_cyrillic(
            value_t!(matches.value_of("hard-soft-sign"), lib::HardSoftSign)?),
        Script::Latin => lib::Converter::to_latin(
            value_t!(matches.value_of("hook-to-left"), lib::HookToLeft)?),
    };

    convert_all_to(&converter, input, sep, &mut io::stdout())?;
    Ok(0)
}

fn convert_all_to(converter: &lib::Converter,
                  iter: &mut Iterator<Item = Res<String>>, sep: &str,
                  out: &mut Write) -> Res<()> {
    for s in iter {
        let converted = converter.convert(&s?);
        out.write(converted.as_bytes())?;
        out.write(sep.as_bytes())?;
    }
    match sep {
        "\n" => (),
        _ => {
            out.write("\n".as_bytes())?;
        },
    };

    Ok(())
}

arg_enum!{
    enum Script {
        Cyrillic,
        Latin
    }
}
