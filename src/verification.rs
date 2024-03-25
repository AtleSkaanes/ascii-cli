use colored::Colorize;

use crate::cli;

pub fn check_base(base: u8) {
    if !(2..=36).contains(&base) {
        println!(
            "{}",
            format!(
                "Base can only be between 2 and 36! The given base was {}",
                base
            )
            .red()
            .bold()
        );
        std::process::exit(1);
    }
}

pub fn under_128(number: u32) {
    if number > 127 {
        println!("{}", format!("{number} is over the ASCII limit of 127!"));
        std::process::exit(1);
    }
}

pub fn get_number(string: String, base: u8) -> u32 {
    check_base(base);

    if let Ok(num) = u32::from_str_radix(&string, base as u32) {
        num
    } else {
        println!(
            "{}",
            format!("\"{}\" is not a valid number in base {}", string, base)
                .red()
                .bold()
        );
        std::process::exit(1);
    }
}

pub fn get_input(input_name: &str) -> String {
    match cli::get_std_in() {
        Ok(input) => input.trim().to_string(),
        Err(_) => {
            println!(
                "{}",
                format!("Please provide a value for \"{}\"", input_name)
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    }
}
