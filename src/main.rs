use types::command::Command;
use crate::database::{schema, record};

mod cli;
mod parser;
mod database;
mod storage;
mod runtime;
mod types;

fn main() {
    println!("Welcome to PebbleDB");
    println!();

    loop {
        let input = cli::shell::read_input();

        if input.is_empty() {
            continue;
        }

        let command = match parser::parser::parse(&input) {
            Ok(command) => command,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };

        match command {
            Command::Database(cmd) => schema::execute_database(cmd),
            Command::Table(cmd) => schema::execute_table(cmd),
            Command::Record(cmd) => record::execute(cmd),
            Command::System(cmd) => runtime::execute(cmd),
        }
    }
}