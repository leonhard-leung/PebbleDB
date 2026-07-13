mod cli;
mod parser;
mod database;
mod storage;

use crate::parser::error::Error;

fn main() {
    println!("Welcome to PebbleDB");
    println!();

    loop {
        let input = cli::shell::read_input();

        let command = parser::parser::parse(&input);

        if command.is_ok() {
            database::engine::execute(command.unwrap());
        } else {
            println!("Error: {}", command.unwrap_err());
        }
    }
}