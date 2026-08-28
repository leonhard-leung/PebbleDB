use crate::cli::color::{CYAN, GREEN, RED, RESET};
use crate::runtime::session::Session;
use crate::shared::error::Error;
use crate::shared::output::Output;
use std::io;
pub use std::io::BufRead;
use std::io::Write;

pub fn cmd_prompt(session: &Session) -> String {
    let prompt = match &session.current_database {
        Some(database) => format!("PebbleDB ({database}) > "),
        None => "PebbleDB > ".to_string(),
    };

    read_input(&prompt)
}

pub fn read_input(prompt: &str) -> String {
    print!("{CYAN}{}{RESET}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_string()
}

pub fn print_out(out: Output) {
    match out {
        Output::DatabaseList(dbs) => {
            for db in dbs {
                println!("{CYAN}{db}{RESET}");
            }
        },
        Output::TablesList(tables) => {
            for table in tables {
                println!("{CYAN}{table}{RESET}");
            }
        },
        Output::TableMetadata(metadata) => {
            for line in metadata {
                println!("{CYAN}{line}{RESET}");
            }
        }
        Output::Message(msg) => println!("{GREEN}{msg}{RESET}"),
    }
}

pub fn print_err(err: Error) {
    println!("{RED}Error: {err}{RESET}");
}