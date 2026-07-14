use crate::types::command::{DatabaseCommand, TableCommand};

/// # Execute Database
pub fn execute_database(command: DatabaseCommand) {
    match command {
        DatabaseCommand::List => list_databases(),
        DatabaseCommand::Create(name) => create_database(&name),
        DatabaseCommand::Drop(name) => drop_database(&name),
        DatabaseCommand::Use(name) => use_database(&name),
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
    println!("Showing databases...");
}

/// # Create Database
fn create_database(name: &str) {
    println!("Database created: {}", name);
}

/// # Drop Database
fn drop_database(name: &str) {
    println!("Database dropped: {}", name);
}

/// Use Database
fn use_database(name: &str) {
    println!("Database selected: {}", name);
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