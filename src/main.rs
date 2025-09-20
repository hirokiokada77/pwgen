mod constants;

use clap::Parser;
use rand::prelude::IndexedRandom;
use std::process;

use crate::constants::{RED_BOLD, RESET};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Password length
    #[clap(short = 'l', long, default_value_t = 20)]
    length: usize,

    /// Character pattern for the password
    #[clap(short = 'p', long, default_value = "A-Za-z0-9")]
    pattern: String,
}

fn main() {
    let args = Args::parse();

    if args.length == 0 {
        eprintln!(
            "{}error{}: the password length cannot be zero",
            RED_BOLD, RESET
        );
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

    println!("{}", password);
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
                eprintln!("{}error{}: invalid character range", RED_BOLD, RESET);
                process::exit(1);
            }
        } else {
            current_chars.push(c);
        }
    }

    chars.extend(current_chars);

    if chars.is_empty() {
        eprintln!(
            "{}error{}: the parsed character set is empty",
            RED_BOLD, RESET
        );
        process::exit(1);
    }

    chars
}
