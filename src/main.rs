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
    cli::shell::print_out("Welcome to PebbleDB\n");
    
    let mut session = Session {
        current_database: None,
    };

    loop {
        let input = cli::shell::cmd_prompt(&session);

        if input.is_empty() {
            continue;
        }

        let command = match parser::parser::parse(&input) {
            Ok(command) => command,
            Err(err) => {
                cli::shell::print_err(err);
                continue;
            }
        };

        match command {
            Command::Database(cmd) => schema::execute_database(cmd, &mut session),
            Command::Table(cmd) => {
                match schema::execute_table(cmd, &mut session) {
                    Ok(_) => (),
                    Err(err) => cli::shell::print_err(err)
                }
            },
            Command::Record(cmd) => record::execute(cmd),
            Command::System(cmd) => system::execute(cmd),
        }
    }
}