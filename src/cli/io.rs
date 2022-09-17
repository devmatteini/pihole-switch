use console::style;
use console::StyledObject;

use pihole_switch::pihole::error::PiHoleError;

pub fn print_success(value: &str) {
    let message = format!("[✓] {}", value);
    print_bold(style(message).green());
}

pub fn print_error(value: &str) {
    let message = format!("[X] {}", value);
    eprint_bold(style(message).red());
}

pub fn print_warning(value: &str) {
    let message = format!("[!] {}", value);
    eprint_bold(style(message).yellow());
}

pub fn print_unknown(value: &str) {
    let message = format!("[?] {}", value);
    eprint_bold(style(message).white());
}

fn print_bold<T: std::fmt::Display>(message: StyledObject<T>) {
    println!("{}", message.bold());
}

fn eprint_bold<T: std::fmt::Display>(message: StyledObject<T>) {
    eprintln!("{}", message.bold());
}

pub fn print_pihole_error(error: PiHoleError) {
    match error {
        PiHoleError::BadRequestOrTokenNotValid => print_error(&error.to_string()),
        PiHoleError::HttpError(_) => print_unknown(&error.to_string()),
        PiHoleError::InvalidResponse => print_unknown(&error.to_string()),
        PiHoleError::NotEnabled => print_warning(&error.to_string()),
        PiHoleError::NotDisabled => print_warning(&error.to_string()),
    }
}
