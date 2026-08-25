//! # Schema
//! Provides the application-facing API for executing database and table commands.
//! Handles command execution, session state, and validation before delegating database
//! operations to the storage layer.

use crate::database::api;
use crate::database::model::{Column, DataType, Output, Table};
use crate::runtime::session::Session;
use crate::types::command::{DatabaseCommand, TableCommand};
use crate::types::error::Error;
use crate::cli;

/// # execute_database
/// Executes a database command and updates the session when required.
pub fn execute_database(
    command: DatabaseCommand, 
    session:&mut Session
) -> Result<Output, Error> {
    match command {
        DatabaseCommand::List => {
            let databases = api::list_databases()?;
            Ok(Output::Databases(databases))
        },
        DatabaseCommand::Create(name) => {
            api::create_database(&name)?;
            Ok(Output::Message(format!("Database created: {}", &name)))
        },
        DatabaseCommand::Drop(name) => {
            api::drop_database(&name)?;
            Ok(Output::Message(format!("Database dropped: {}", &name)))
        },
        DatabaseCommand::Use(name) => {
            api::use_database(&name, session)?;

            if name.eq_ignore_ascii_case("none") {
                Ok(Output::Message("Database deselected".to_string()))
            } else {
                Ok(Output::Message(format!("Database selected: {}", &name)))
            }
        },
    }
}

/// # execute_table
/// Executes a table command within the database selected in the current session.
pub fn execute_table(
    command: TableCommand, 
    session:&mut Session
) -> Result<Output, Error> {
    let Some(db_name) = session.current_database.as_ref() else {
        return Err(Error::NoSelectedDatabase);
    };

    match command {
        TableCommand::List => {
            let tables = api::list_tables(db_name)?;
            Ok(Output::Tables(tables))
        },
        TableCommand::Create(table_name) => {
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
                    input = cli::shell::
                    read_input("Add Another Column? <y/n>: ")
                        .to_lowercase();

                    if input == "y" || input == "n" { break; }
                }

                if input == "y" {
                    continue;
                }
                break
            }

            let table = Table {
                name: table_name.to_string(),
                columns,
                row_count: 0
            };

            api::create_table(table, &db_name)?;
            Ok(Output::Message(format!("Table created: {}", &table_name)))
        },
        TableCommand::Drop(name) => {
            api::drop_table(&name, &db_name)?;
            Ok(Output::Message(format!("Table dropped: {}", &name)))
        }
        TableCommand::Describe(name) => {
            let data = api::describe_table(&name, &db_name)?;
            Ok(Output::TableMetadata(data))
        },
    }
}
