use ascii_cli::*;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    match args.command.unwrap_or_default() {
        cli::SubCommands::Table { base } => table::show_table(args.base.unwrap_or(base)),
        cli::SubCommands::ToChar {
            number,
            base,
            ignore_specials,
        } => convert::to_char(number, base, ignore_specials),
        cli::SubCommands::ToNum { char, base } => convert::to_num(char, args.base.unwrap_or(base)),
        cli::SubCommands::WhatIs {
            character,
            base,
            special,
        } => info::what_is(character, args.base.unwrap_or(base), special),
    }
    println!();
}
