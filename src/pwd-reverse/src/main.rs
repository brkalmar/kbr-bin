use std::{env, path, process};

fn main() {
    let separator = match env::args().len() {
        1 => path::MAIN_SEPARATOR.to_string(),
        2 => env::args().nth(1)
            .expect("args() is of len 2, but there is no argument 1"),
        _ => {
            eprintln!("usage: {} [SEPARATOR]", env::args().nth(0)
                      .unwrap_or("<program>".into()));
            process::exit(2);
        },
    };

    if let Ok(mut dir) = env::current_dir() {
        if dir.file_name().is_none() {
            println!("{}", separator);
            return;
        }
        loop {
            match dir.file_name() {
                None => break,
                Some(component) => {
                    let component = match component.to_str() {
                        Some(component) => component,
                        None => {
                            eprintln!("\nerror: next component is not valid \
                                       Unicode: {:?}", component);
                            process::exit(1);
                        }
                    };
                    match dir.parent() {
                        Some(parent) if parent != path::Path::new("") => {
                            print!("{}{}", component, separator)
                        },
                        Some(_) |
                        None => print!("{}", component),
                    };
                }
            }
            dir.pop();
        }
        println!();
    }
}
