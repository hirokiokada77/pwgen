use crate::constants::{RED_BOLD, RESET};

pub(crate) fn print_error(message: &str) {
    eprintln!("{RED_BOLD}error:{RESET} {message}");
}
