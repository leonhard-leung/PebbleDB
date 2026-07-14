use crate::parser::command::Action;
use crate::parser::command::Target;

/// # Execute
/// Executes an action sent by the parser
pub fn execute(task: Action) {
    match task {
        Action::List(target_type) => show_objects(target_type),
        Action::Create(object, name) => create_object(object, &name),
        Action::Delete(object, name) => delete_object(object, &name),
        Action::Use(object, name) => use_object(object, &name),
        Action::Exit => std::process::exit(0),
    }
}

/// # Show Objects
/// Lists either the databases or the tables in the current database
pub fn show_objects(target_type: Target) {
    println!("list all {}", target_type)
}

/// # Create Object
/// Creates either a database or a table
///
pub fn create_object(object: Target, name: &str) {
    match object {
        Target::Database => println!("Database created: {}", name),
        Target::Table => println!("Table created: {}", name),
    }
}

/// # Delete Object
/// Drops either a database or a table in the current database
pub fn delete_object(object: Target, name: &str) {
    match object {
        Target::Database => println!("Database dropped: {}", name),
        Target::Table => println!("Table dropped: {}", name),
    }
}

/// # Use Object
/// Selects a object to be active
pub fn use_object(object: Target, name: &str) {
    match object {
        Target::Database => println!("Database selected: {}", name),
        Target::Table => println!("Table selected: {}", name),
    }
}