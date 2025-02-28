use clap::{Parser, Subcommand};
use std::io::BufRead;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /// Shows the Ascii table
    Table {
        /// The numbers base, deafults to 16 (Hexadecimal) [max is 36]
        #[arg(short, long, default_value_t = 16)]
        base: u8,
    },
    ToChar {
        /// The number(s) to be converted to a character
        #[clap(num_args = 1.., value_delimiter = ' ')]
        number: Option<Vec<String>>,
        /// The numbers base, deafults to 16 (Hexadecimal) [max is 36]
        #[arg(short, long, default_value_t = 16)]
        base: u8,
        /// If it should print special characters as their name, or make them act as usual
        /// (Example: 7 is BEL if false, or it will make a bell sound if true)
        #[arg(short, long)]
        ignore_specials: bool,
    },
    ToNum {
        /// The char(s) to be converted to a character
        #[clap(num_args = 1.., value_delimiter = ' ')]
        char: Option<Vec<String>>,
        /// The numbers base, deafults to 16 (Hexadecimal) [max is 36]
        #[arg(short, long, default_value_t = 16)]
        base: u8,
    },
    WhatIs {
        /// The number of the char you want to know more about, or the special characters name if
        /// its a special character you want to know about (If so, use the -s flag)
        character: Option<String>,
        /// The numbers base, deafults to 16 (Hexadecimal) [max is 36]
        #[arg(short, long, default_value_t = 16)]
        base: u8,
        /// Set this flag if the string is the name of a special character, like CR
        #[arg(short, long)]
        special: bool,
    },
}

impl Default for SubCommands {
    fn default() -> Self {
        Self::Table { base: 16 }
    }
}

pub fn get_std_in() -> String {
    std::io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| {
            acc + &line.unwrap_or_default() + "\n"
        })
}
