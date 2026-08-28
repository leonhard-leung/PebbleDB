use crate::runtime::session::Session;
use crate::runtime::system;
use application::executor;
use shared::command::Command;
use shared::output::Output;

mod cli;
mod parser;
mod database;
mod storage;
mod runtime;
mod shared;
mod constants;
pub mod application;

fn main() {
    cli::shell::print_out(Output::Message("Welcome to PebbleDB!".to_string()));
    
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
            Command::Database(cmd) => {
                match executor::execute_database(cmd, &mut session) {
                    Ok(output) => cli::shell::print_out(output),
                    Err(err) => cli::shell::print_err(err),
                }
            },
            Command::Table(cmd) => {
                match executor::execute_table(cmd, &mut session) {
                    Ok(output) => cli::shell::print_out(output),
                    Err(err) => cli::shell::print_err(err),
                }
            },
            Command::Record(cmd) => {
                match executor::execute_record(cmd, &mut session) {
                    Ok(output) => cli::shell::print_out(output),
                    Err(err) => cli::shell::print_err(err),
                }
            },
            Command::System(cmd) => system::execute(cmd),
        }
    }
}