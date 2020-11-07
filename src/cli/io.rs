use ansi_term::{Color, Style};

use pihole_switch::pihole::PiHoleError;

pub fn print_success(value: &str) {
    print_bold(Color::Green, &format!("[✓] {}", value));
}

pub fn print_error(value: &str) {
    eprint_bold(Color::Red, &format!("[X] {}", value));
}

pub fn print_warning(value: &str) {
    eprint_bold(Color::Yellow, &format!("[!] {}", value));
}

pub fn print_unknown(value: &str) {
    eprint_bold(Color::White, &format!("[?] {}", value));
}

fn print_bold(color: Color, message: &str) {
    println!("{}", Style::new().bold().fg(color).paint(message));
}

fn eprint_bold(color: Color, message: &str) {
    eprintln!("{}", Style::new().bold().fg(color).paint(message));
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
