use pihole_switch::pihole::PiHoleError;

pub fn print_success(value: &str) {
    println!("[✓] {}", value);
}

pub fn print_error(value: &str) {
    eprintln!("[X] {}", value);
}

pub fn print_pihole_error(error: PiHoleError) {
    match error {
        PiHoleError::BadRequestOrTokenNotValid => print_error(&error.to_string()),
        PiHoleError::HttpError => eprintln!("[?] {}", error),
        PiHoleError::InvalidResponse => eprintln!("[?] {}", error),
        PiHoleError::NotEnabled => eprintln!("[!] {}", error),
        PiHoleError::NotDisabled => eprintln!("[!] {}", error),
    }
}
