use crate::constants::{RED_BOLD, RESET};

pub(crate) fn print_error(message: &str) {
    eprintln!("{}error:{} {}", RED_BOLD, RESET, message);
}
