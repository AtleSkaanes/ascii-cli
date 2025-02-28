use std::collections::HashMap;

use colored::Colorize;
use radix_fmt::radix;

use crate::verification;

pub fn show_table(base: u8) {
    verification::check_base(base);
    for row in 0..16 {
        for col in 0..8 {
            let char_num = row + col * 16;
            let char_set = get_char(char_num, true);
            let char_string = if char_set.1 {
                char_set.0.italic().bold()
            } else {
                char_set.0.bold()
            };
            print!("{:>8}", format!("{:#}", radix(char_num, base)).blue());
            print!(" {:<5}", char_string);
        }
        println!();
    }
}

pub fn get_char(number: u32, specials_as_str: bool) -> (String, bool) {
    if specials_as_str {
        let specials = get_special_chars();
        if let Some(special_char) = specials.get(&number) {
            return (special_char.to_string(), true);
        }
    }

    (char::from_u32(number).unwrap_or('E').to_string(), false)
}

pub fn get_special_chars() -> HashMap<u32, String> {
    HashMap::from([
        (0, "NUL".to_owned()),
        (1, "SOH".to_owned()),
        (2, "STX".to_owned()),
        (3, "ETX".to_owned()),
        (4, "EOT".to_owned()),
        (5, "ENQ".to_owned()),
        (6, "ACK".to_owned()),
        (7, "BEL".to_owned()),
        (8, "BS".to_owned()),
        (9, "HT".to_owned()),
        (10, "LF".to_owned()),
        (11, "VT".to_owned()),
        (12, "FF".to_owned()),
        (13, "CR".to_owned()),
        (14, "SO".to_owned()),
        (15, "SI".to_owned()),
        (16, "DLE".to_owned()),
        (17, "DC1".to_owned()),
        (18, "DC2".to_owned()),
        (19, "DC3".to_owned()),
        (20, "DC4".to_owned()),
        (21, "NAK".to_owned()),
        (22, "SYN".to_owned()),
        (23, "ETB".to_owned()),
        (24, "CAN".to_owned()),
        (25, "EM".to_owned()),
        (26, "SUB".to_owned()),
        (27, "ESC".to_owned()),
        (28, "FS".to_owned()),
        (29, "GS".to_owned()),
        (30, "RS".to_owned()),
        (31, "US".to_owned()),
        (127, "DEL".to_owned()),
    ])
}
