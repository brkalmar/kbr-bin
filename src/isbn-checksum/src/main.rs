#[macro_use]
extern crate lazy_static;

use std::collections;
use std::env;
use std::error;
use std::fmt;
use std::iter;
use std::process;

fn main() {
    process::exit(
        match run() {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("{}", e);
                1
            }
        }
    )
}

fn run() -> Result<(), Error> {
    let mut args = env::args().skip(1);
    let digits = match args.len() {
        1 => args.next().unwrap(),
        _ => return Err(Error::new(
            format!("usage: {} ISBN-DIGITS",
                    env::args().next().unwrap_or("<program>".into()))
        )),
    };
    let digits = digits.chars()
        .filter_map(char_to_digit);

    const LEN_MAX: usize = 12;
    let check_digit = match digits.clone().take(LEN_MAX + 1).count() {
        9 => check_digit_10(digits.clone()),
        12 => check_digit_13(digits.clone()),
        _ => return Err(Error::new(
            "invalid ISBN length: 9 or 12 required".into()
        )),
    };
    let isbn: String = digits
        .chain(iter::once(check_digit))
        .map(|d| digit_to_char(d).unwrap())
        .collect();
    println!("{}", isbn);

    Ok(())
}

fn check_digit_10<I: Iterator<Item = u32>>(digits: I) -> u32 {
    let sum: u32 = digits.enumerate()
        .map(|(i, digit)| {
            let weight = 10 - i as u32;
            weight * digit
        })
        .sum();
    (11 - (sum % 11)) % 11
}

fn check_digit_13<I: Iterator<Item = u32>>(digits: I) -> u32 {
    let sum: u32 = digits.enumerate()
        .map(|(i, digit)| {
            let weight = (i as u32 % 2) * 2 + 1;
            weight * digit
        })
        .sum();
    (10 - (sum % 10)) % 10
}

const DIGIT_TO_CHAR: [char; 11] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'X'
];

lazy_static! {
    static ref CHAR_TO_DIGIT: collections::HashMap<char, u32>
        = DIGIT_TO_CHAR.iter()
        .enumerate()
        .map(|(i, &c)| (c, i as u32))
        .collect();
}

fn digit_to_char(d: u32) -> Option<char> {
    let d = d as usize;
    if d < DIGIT_TO_CHAR.len() {
        Some(DIGIT_TO_CHAR[d])
    } else {
        None
    }
}

fn char_to_digit(c: char) -> Option<u32> {
    CHAR_TO_DIGIT.get(&c).map(|&d| d)
}

#[derive(Debug)]
struct Error {
    description: String,
}

impl Error {
    fn new(description: String) -> Self {
        Self {
            description,
        }
    }
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description.fmt(f)
    }
}
