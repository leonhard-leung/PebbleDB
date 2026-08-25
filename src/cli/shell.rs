use std::io;
pub use std::io::{BufRead};
use std::io::Write;
use crate::database::model::Output;
use crate::runtime::session::Session;
use crate::shared::error::Error;

pub fn cmd_prompt(session: &Session) -> String {
    let prompt = match &session.current_database {
        Some(database) => format!("PebbleDB ({database}) > "),
        None => "PebbleDB > ".to_string(),
    };

    read_input(&prompt)
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_string()
}

pub fn print_out(out: Output) {
    match out {
        Output::Databases(dbs) => {
            for db in dbs {
                println!("{}", db);
            }
        },
        Output::Tables(tables) => {
            for table in tables {
                println!("{}", table);
            }
        },
        Output::TableMetadata(metadata) => {
            for line in metadata {
                println!("{}", line);
            }
        }
        Output::Message(msg) => println!("{}", msg),
    }
}

pub fn print_err(err: Error) {
    println!("Error: {}", err)
}