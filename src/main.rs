use clap::Parser;

mod cli;
mod convert;
mod info;
mod table;
mod verification;

fn main() {
    let args = cli::Args::parse();

    match args.command {
        cli::SubCommands::Table { base } => table::show_table(base),
        cli::SubCommands::ToChar {
            number,
            base,
            ignore_specials,
        } => convert::to_char(number, base, ignore_specials),
        cli::SubCommands::ToNum { char, base } => convert::to_num(char, base),
        cli::SubCommands::WhatIs {
            character,
            base,
            special,
        } => info::what_is(character, base, special),
    }
    println!("");
}
