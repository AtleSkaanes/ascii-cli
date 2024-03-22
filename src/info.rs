use std::collections::HashMap;

use colored::Colorize;
use radix_fmt::radix;

use crate::{table, verification};

pub fn what_is(string: String, base: u8, is_special: bool) {
    verification::check_base(base);

    let mut special = is_special;

    let mut character = String::new();
    let mut name = String::new();
    let mut description = String::new();
    let mut category = String::new();

    let mut number = 0;

    if special {
        character = string.to_uppercase();
    } else {
        let num = verification::get_number(string.clone(), base);
        verification::under_128(num);
        number = num;
        if let Some(special_char) = table::get_special_chars().get(&num) {
            character = special_char.to_owned();
            special = true;
        } else {
            character = char::from_u32(num).unwrap().to_string();
        }
    }

    if special {
        if let Some(char_set) = get_special_descriptions().get(&character.as_str()) {
            name = format!("{} - {}", character, char_set.0);
            description = char_set.1.to_owned();
            number = char_set.2 as u32;
            category = "Control Character".to_owned();
        } else {
            println!(
                "{}",
                format!("\"{}\" is not a Control Character!", character)
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    } else {
        if (32..=47).contains(&number)
            || (58..=64).contains(&number)
            || (91..=96).contains(&number)
            || (123..=126).contains(&number)
        {
            category = "Special Characters".to_owned();
            description = "A special character, typically used for punctuations".to_owned();
        } else if (30..=39).contains(&number) {
            category = "Numbers".to_owned();
            description = "A number in base 10 (0-9)".to_owned();
        } else if (65..=90).contains(&number) {
            category = "Uppercase letter".to_owned();
            description = "An Uppercase letter in the english alphabet (A-Z)".to_owned();
        } else if (97..=122).contains(&number) {
            category = "Lowercase letter".to_owned();
            description = "A lowercase letter in the english alphabet (a-z)".to_owned();
        }
        name = character;
    }

    println!(
        "{}",
        format!("{}: {}", radix(number, base), name).blue().bold()
    );
    println!("{} {}", "Category:".blue().bold(), category.italic());
    println!("{} {}", "Description:".blue().bold(), description.italic());
}

fn get_special_descriptions() -> HashMap<&'static str, (&'static str, &'static str, u8)> {
    HashMap::from([
        ("NUL", ("NULL character", "The null character prompts the device to do nothing", 0)),
        ("SOH", ("Start Of Heading", "Initiates a header", 1)),
        ("STX", ("Start Of Text", "Ends the header and marks the beginning of a message", 2)),
        ("ETX", ("End Of Text", "Indicates the end of the message", 3)),
        ("EOT", ("End Of Transmission", "Marks the end of a completes transmission", 4)),
        ("ENQ", ("Enquiry", "A request that requires a response", 5)),
        ("ACK", ("Acknowledge", "Gives a positive answer to the request", 6)),
        ("BEL", ("Bell", "Triggers a beep", 7)),
        ("BS", ("Backspace", "Lets the cursor move back one step", 8)),
        ("HT", ("Horizontal tab", "A horizontal tab that moves the cursor within a row to the next predefined position", 9)),
        ("LF", ("Line Feed", "Causes the cursor to jump to the next line", 10)),
        ("VT", ("Vertical Tab", "The vertical tab lets the cursor jump to a predefined line", 11)),
        ("FF", ("Form Feed", "Requests a page break", 12)),
        ("CR", ("Carriage Return", "Moves the cursor back to the first position of the line", 13)),
        ("SO", ("Shift Out", "Switches to a special presentation", 14)),
        ("SI", ("Shift In", "Switches the display back to the normal state", 15)),
        ("DLE", ("Data Link Escape", "Changes the meaning of the following characters", 16)),
        ("DC1", ("Device Control", "Control characters assigned depending on the device used", 17)),
        ("DC2", ("Device Control", "Control characters assigned depending on the device used", 18)),
        ("DC3", ("Device Control", "Control characters assigned depending on the device used", 19)),
        ("DC4", ("Device Control", "Control characters assigned depending on the device used", 20)),
        ("NAK", ("Negative Acknowledge", "Negative response to a request", 21)),
        ("SYN", ("Synchronous Idle", "Synchronizes a data transfer, even if no signals are transmitted", 22)),
        ("ETB", ("End of Transmission Block", "Marks the end of a transmission block", 23)),
        ("CAN", ("Cancel", "Makes it clear that a transmission was faulty and the data must be discarded", 24)),
        ("EM", ("End of Medium", "Indicates the end of the storage medium", 25)),
        ("SUB", ("Substitute", "Replacement for a faulty sign", 26)),
        ("ESC", ("Escape", "Initiates an escape sequence and thus gives the following characters a special meaning", 27)),
        ("FS", ("File Separator", "Marks the separation of logical data blocks and is hierarchically ordered: file as the largest unit, file as the smallest unit", 28)),
        ("GS", ("Group Separator", "Marks the separation of logical data blocks and is hierarchically ordered: file as the largest unit, file as the smallest unit", 29)),
        ("RS", ("Record Separator", "Marks the separation of logical data blocks and is hierarchically ordered: file as the largest unit, file as the smallest unit", 30)),
        ("US", ("Unit Separator", "Marks the separation of logical data blocks and is hierarchically ordered: file as the largest unit, file as the smallest unit", 31)),
        ("DEL", ("Delete", "Deletes a character. Since this control character consists of the same number on all positions, during the typewriter era it was possible to invalidate another character by punching out all the positions", 127)),
    ])
}
