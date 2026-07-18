use std::io;
pub use std::io::{BufRead};
use std::io::Write;
use crate::runtime::session::Session;

pub fn read_input(session: &Session) -> String {
    // print
    let prompt = match &session.current_database {
        Some(database) => format!("PebbleDB ({database}) > "),
        None => "PebbleDB > ".to_string(),
    };

    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn print_out(out: &str) {
    println!("{}", out);
}