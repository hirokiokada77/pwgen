mod constants;
mod util;

use clap::Parser;
use rand::prelude::IndexedRandom;
use std::process;

use crate::util::print_error;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Password length
    #[clap(short = 'l', long, default_value_t = 20)]
    length: usize,

    /// Character pattern for the password
    #[clap(short = 'p', long, default_value = "A-Za-z0-9")]
    pattern: String,

    /// Do not print the trailing newline character
    #[clap(short = 'n', long, default_value_t = false)]
    no_newline: bool,
}

fn main() {
    let args = Args::parse();

    if args.length == 0 {
        print_error("the password length cannot be zero");
        process::exit(1);
    }

    let pattern_chars = get_pattern_chars(&args.pattern);

    let mut rng = rand::rng();
    let mut password = String::new();

    for _ in 0..args.length {
        if let Some(c) = pattern_chars.choose(&mut rng) {
            password.push(*c);
        }
    }

    if args.no_newline {
        print!("{}", password);
    } else {
        println!("{}", password);
    }
}

fn get_pattern_chars(pattern: &str) -> Vec<char> {
    let mut chars = Vec::new();
    let inner_pattern = pattern.trim_matches(|c| c == '[' || c == ']');
    let mut current_chars = Vec::new();
    let mut chars_iter = inner_pattern.chars().peekable();

    while let Some(c) = chars_iter.next() {
        if c == '-' && !current_chars.is_empty() && chars_iter.peek().is_some() {
            let start = current_chars.pop().unwrap();
            let end = chars_iter.next().unwrap();

            if start < end {
                for ch in start..=end {
                    current_chars.push(ch);
                }
            } else {
                print_error("invalid character range");
                process::exit(1);
            }
        } else {
            current_chars.push(c);
        }
    }

    chars.extend(current_chars);

    if chars.is_empty() {
        print_error("the parsed character set is empty");
        process::exit(1);
    }

    chars
}
