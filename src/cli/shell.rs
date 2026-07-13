use std::io;
pub use std::io::{BufRead};
use std::io::Write;

pub fn read_input() -> String {
    // print
    print!("PebbleDB > ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn print_out(out: &str) {
    println!("{}", out);
}