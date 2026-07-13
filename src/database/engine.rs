use crate::parser::command::Action;
use crate::parser::command::Target;
use crate::parser::error::Error;

pub fn execute(task: Action) {
    match task {
        Action::Create(object, name) => create(object, &name),
        Action::Exit => std::process::exit(0),
    }
}

pub fn create(object: Target, name: &str) {
    match object {
        Target::Database => println!("Database created: {}", name),
        Target::Table => println!("Table created: {}", name),
    }
}