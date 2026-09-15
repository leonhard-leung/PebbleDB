use std::fs;
use crate::runtime::session::{Session, ServerState};
use crate::runtime::system::execute_system;
use crate::server::executor::execute_server;
use application::executor;
use shared::command::Command;
use shared::output::Output;
use crate::constants::system::root;

mod cli;
mod parser;
mod database;
mod storage;
mod runtime;
mod shared;
mod constants;
pub mod application;
pub mod server;

#[tokio::main]
async fn main() {
    let root = root();

    if !root.exists() {
        fs::create_dir_all(&root)
            .expect("Failed to create PebbleDB data directory");
    }

    cli::shell::print_out(Output::Message("Welcome to PebbleDB!".to_string()));

    let mut session = Session {
        current_database: None,
        server: ServerState {
            running: false,
            shutdown: None,
        }
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
            Command::System(cmd) => {
                let output = execute_system(cmd);
                match output {
                    Output::ExitApplication => {
                        cli::shell::print_out(output);
                        std::process::exit(0)
                    },
                    _ => cli::shell::print_out(output),
                }
            },
            Command::Server(cmd) => {
                match execute_server(cmd, &mut session) {
                    Ok(output) => cli::shell::print_out(output),
                    Err(err) => cli::shell::print_err(err),
                }
            },
        }
    }
}