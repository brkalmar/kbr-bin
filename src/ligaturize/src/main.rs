#[macro_use]
extern crate clap;
extern crate unicode_normalization;

use std::error;
use std::io::prelude::*;
use std::io;
use unicode_normalization::UnicodeNormalization;

type Res<T> = Result<T, Box<error::Error>>;

fn main() {
    std::process::exit(match run() {
        Err(e) => {
            eprintln!("{}", e);
            1
        },
        Ok(status) => status,
    })
}

fn run() -> Res<i32> {
    let _matches = clap::App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches_safe()?;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let ligaturized = ligaturize(&line?);
        io::stdout().write(ligaturized.as_bytes())?;
    }
    Ok(0)
}

fn ligaturize(s: &String) -> String {
    let ligatures = {
        let mut ligatures = LIGATURES.clone();
        // replace longer strings first, otherwise if a is a prefix of b, b will
        // never be replaced with its replacement
        ligatures.sort_unstable_by_key(|&(from, _)| from.chars().count());
        ligatures.reverse();
        ligatures
    };
    let mut ret: String = s.nfc().collect();
    for &(from, to) in ligatures.iter() {
        ret = ret.replace(from, to);
    }
    ret
}

const LIGATURES: [(&str, &str); 11] = [
    ("IJ", "Ĳ"),
    ("OE", "Œ"),
    ("ff", "ﬀ"),
    ("ffi", "ﬃ"),
    ("ffl", "ﬄ"),
    ("fi", "ﬁ"),
    ("fl", "ﬂ"),
    ("ij", "ĳ"),
    ("oe", "œ"),
    ("st", "ﬆ"),
    ("ſt", "ﬅ"),
];
