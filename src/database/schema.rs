use crate::constants::format::COLUMN_DEFINITION_SIZE;
use crate::database::model::{Column, DataType, Table};
use crate::runtime::session::Session;
use crate::types::command::{DatabaseCommand, TableCommand};
use crate::{cli, storage};
use crate::types::error::Error;

/// # Execute Database
pub fn execute_database(command: DatabaseCommand, session:&mut Session , ) -> Result<(), Error> {
    match command {
        DatabaseCommand::List => {
            let databases = list_databases()?;

            for db in databases {
                cli::shell::print_out(&db);
            }
        },
        DatabaseCommand::Create(name) => {
            create_database(&name)?;
            cli::shell::print_out(&format!("Database created: {}", name));
        },
        DatabaseCommand::Drop(name) => {
            drop_database(&name)?;
            cli::shell::print_out(&format!("Database dropped: {}", name));
        },
        DatabaseCommand::Use(name) => {
            use_database(&name, session)?;

            if name.eq_ignore_ascii_case("none") {
                cli::shell::print_out("No database selected");
            } else {
                cli::shell::print_out(&format!("Database used: {}", name));
            }
        },
    }
    Ok(())
}

/// # Execute Table
pub fn execute_table(command: TableCommand, session:&mut Session) -> Result<(), Error> {
    let Some(db_name) = session.current_database.as_ref() else {
        return Err(Error::NoSelectedDatabase);
    };

    match command {
        TableCommand::List => list_tables(),
        TableCommand::Create(name) => create_table(&name, &db_name),
        TableCommand::Drop(name) => drop_table(&name),
        TableCommand::Describe(name) => describe_table(&name),
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # DATABASE SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # List Databases
fn list_databases() -> Result<Vec<String>, Error> {
    storage::filesystem::list_databases()
}

/// # Create Database
fn create_database(name: &str) -> Result<(), Error> {
    storage::filesystem::create_database(name)?;
    Ok(())
}

/// # Drop Database
fn drop_database(name: &str) -> Result<(), Error> {
    storage::filesystem::drop_database(name)?;
    Ok(())
}

/// # Use Database
fn use_database(name: &str, session: &mut Session) -> Result<(), Error> {
    if name == "none" {
        session.current_database = None;

    }

    let list = storage::filesystem::list_databases()?;

    if list.iter().any(| db | db.eq_ignore_ascii_case(name)) {
        session.current_database = Some(name.to_string());
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # TABLE SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # List Tables
fn list_tables() {
    println!("Showing tables...");
}

/// # Create Table
fn create_table(table_name: &str, db_name: &str) {
    // column details
    let mut columns: Vec<Column> = Vec::new();
    loop {
        // column name
        let column_name = cli::shell::read_input("Column Name: ");

        // column data type
        let data_type: DataType;
        loop {
            let input = cli::shell::read_input("Column Type: ");
            match input.to_lowercase().as_str() {
                "int" => data_type = DataType::Integer,
                "float" => data_type = DataType::Float,
                "boolean" => data_type = DataType::Boolean,
                "text" => data_type = DataType::Text,
                _ => continue,
            }
            break
        }

        // push column to vector
        let column = Column {
            name: column_name,
            data_type,
        };
        columns.push(column);

        // add another column
        let mut input: String;
        loop {
            input = cli::shell::read_input("Add Another Column? <y/n>: ");

            if input.to_lowercase() == "y" || input.to_lowercase() == "n" { break; }
        }

        if input.to_lowercase() == "y" {
            continue;
        }
        break
    }

    let table = Table {
        name: table_name.to_string(),
        columns,
        row_count: 0,
    };

    match storage::filesystem::create_table(table, db_name) {
        Ok(()) => println!("Table created: {}", table_name),
        Err(err) => println!("Error creating table: {}", err),
    };
}

/// # Drop Table
fn drop_table(name: &str) {
    println!("Table dropped: {}", name);
}

/// # Describe Table
fn describe_table(name: &str) {
    println!("Table description: {}", name);
}