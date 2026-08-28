use crate::cli::format::{BOLD_FACE, CYAN, GREEN, RED, RESET, YELLOW};
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
                println!("{}", db);
            }
        },
        Output::TablesList(tables) => {
            for table in tables {
                println!("{}", table);
            }
        },
        Output::TableMetadata(metadata) => {
            println!("{BOLD_FACE}{} DETAILS{RESET}", metadata[0].to_uppercase());
            println!("  Column Count: {CYAN}{}{RESET}", metadata[1]);
            println!("  Record Count: {CYAN}{}{RESET}", metadata[2]);
            println!("{BOLD_FACE}COLUMN DETAILS{RESET}");

            for index in (3..metadata.len() - 1).step_by(2) {
                println!("  {BOLD_FACE}Column {}{RESET}", (index - 2) / 2 + 1);
                println!("    | Column Name: {CYAN}{}{RESET}", metadata[index], );
                println!("    | Column Type: {CYAN}{}{RESET}", metadata[index + 1]);
            }
        }
        Output::Message(msg) => println!("{GREEN}{msg}{RESET}"),
    }
}

pub fn print_warning(warning: String) {
    println!("{YELLOW}{warning}{RESET}");
}

pub fn print_err(err: Error) {
    println!("{RED}Error: {err}{RESET}");
}