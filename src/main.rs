use types::command::Command;
use crate::database::{schema, record};
use crate::runtime::{system};
use crate::runtime::session::Session;

mod cli;
mod parser;
mod database;
mod storage;
mod runtime;
mod types;
mod constants;

fn main() {
    println!("Welcome to PebbleDB");
    println!();
    
    let mut session = Session {
        current_database: None,
    };

    loop {
        let input = cli::shell::read_input(&session);

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
            Command::Database(cmd) => schema::execute_database(&mut session, cmd),
            Command::Table(cmd) => schema::execute_table(cmd),
            Command::Record(cmd) => record::execute(cmd),
            Command::System(cmd) => system::execute(cmd),
        }
    }
}