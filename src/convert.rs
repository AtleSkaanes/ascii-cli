use colored::Colorize;
use radix_fmt::radix;

use crate::{table, verification};

pub fn to_char(nums: Vec<String>, base: u8, ignore_specials: bool) {
    verification::check_base(base);
    for num in nums {
        let char_num = verification::get_number(num, base);
        verification::under_128(char_num);
        let c = table::get_char(char_num, !ignore_specials);
        if c.1 {
            print!("{}", c.0.blue().italic());
        } else {
            print!("{}", c.0);
        }
    }
}

pub fn to_num(chars: Vec<String>, base: u8) {
    verification::check_base(base);
    for c in chars.join(" ").chars() {
        let mut string = format!("{:#}", radix(c as u32, base));
        if c == ' ' {
            string = format!(" {} ", string.blue().italic());
        }
        print!("{} ", string);
    }
}
