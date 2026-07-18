use crate::runtime::session::Session;
use crate::types::command::{DatabaseCommand, TableCommand};
use crate::storage;

/// # Execute Database
pub fn execute_database(session:&mut Session , command: DatabaseCommand) {
    match command {
        DatabaseCommand::List => list_databases(),
        DatabaseCommand::Create(name) => create_database(&name),
        DatabaseCommand::Drop(name) => drop_database(&name),
        DatabaseCommand::Use(name) => use_database(&name, session),
    }
}

/// # Execute Table
pub fn execute_table(command: TableCommand) {
    match command {
        TableCommand::List => list_tables(),
        TableCommand::Create(name) => create_table(&name),
        TableCommand::Drop(name) => drop_table(&name),
        TableCommand::Describe(name) => describe_table(&name),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # DATABASE SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # List Databases
fn list_databases() {
    match storage::filesystem::list_databases() {
        Ok(list) => {
            for entry in list {
                println!("{}", entry);
            }
        },
        Err(err) => println!("Error listing databases: {}", err),
    }
}

/// # Create Database
fn create_database(name: &str) {
    match storage::filesystem::create_database(name) {
        Ok(()) => println!("Database created: {}", name),
        Err(err) => println!("Error creating database: {}", err),
    }
}

/// # Drop Database
fn drop_database(name: &str) {
    match storage::filesystem::drop_database(name) {
        Ok(()) => println!("Database dropped: {}", name),
        Err(err) => println!("Error dropping database: {}", err),
    }
}

/// Use Database
fn use_database(name: &str, session: &mut Session) {
    if name == "none" {
        session.current_database = None;
        println!("No database selected")
    }

    match storage::filesystem::list_databases() {
        Ok(list) => {
            if list.iter().any(|db| db == name) {
                session.current_database = Some(name.to_string());
                println!("Database selected: {}", name);
            }
        },
        Err(err) => println!("Error using database: {}", err),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # TABLE SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # List Tables
fn list_tables() {
    println!("Showing tables...");
}

/// # Create Table
fn create_table(name: &str) {
    println!("Table created: {}", name);
}

/// # Drop Table
fn drop_table(name: &str) {
    println!("Table dropped: {}", name);
}

/// # Describe Table
fn describe_table(name: &str) {
    println!("Table description: {}", name);
}