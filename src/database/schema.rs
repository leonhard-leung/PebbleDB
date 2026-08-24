use crate::database::model::{Column, DataType, Table};
use crate::runtime::session::Session;
use crate::types::command::{DatabaseCommand, TableCommand};
use crate::types::error::Error;
use crate::{cli, storage};

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
            cli::shell::print_out(&format!("Database created: {}", &name));
        },
        DatabaseCommand::Drop(name) => {
            drop_database(&name)?;
            cli::shell::print_out(&format!("Database dropped: {}", &name));
        },
        DatabaseCommand::Use(name) => {
            use_database(&name, session)?;

            if name.eq_ignore_ascii_case("none") {
                cli::shell::print_out("No database selected");
            } else {
                cli::shell::print_out(&format!("Database used: {}", &name));
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
        TableCommand::List => {
            let tables = list_tables(db_name)?;

            for table in tables {
                cli::shell::print_out(&table);
            }
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

            create_table(table, &db_name)?;
            cli::shell::print_out(&format!("Table created: {}", &table_name));
        },
        TableCommand::Drop(name) => {
            drop_table(&name, &db_name)?;
            cli::shell::print_out(&format!("Table dropped: {}", &name));
        }
        TableCommand::Describe(name) => {
            let data = describe_table(&name, &db_name)?;
            for contents in data {
                cli::shell::print_out(&contents);
            }
        },
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # DATABASE SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # List Databases
fn list_databases() -> Result<Vec<String>, Error> {
    storage::database::list_databases()
}

/// # Create Database
fn create_database(name: &str) -> Result<(), Error> {
    storage::database::create_database(name)
}

/// # Drop Database
fn drop_database(name: &str) -> Result<(), Error> {
    storage::database::drop_database(name)
}

/// # Use Database
fn use_database(name: &str, session: &mut Session) -> Result<(), Error> {
    if name == "none" {
        session.current_database = None;

    }

    let list = storage::database::list_databases()?;
    if list.iter().any(| db | db.eq_ignore_ascii_case(name)) {
        session.current_database = Some(name.to_string());
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # TABLE SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # List Tables
fn list_tables(db_name: &str) -> Result<Vec<String>, Error> {
    storage::table::list_tables(db_name)
}

/// # Create Table
fn create_table(table: Table, db_name: &str) -> Result<(), Error>{
    storage::table::create_table(table, db_name)
}

/// # Drop Table
fn drop_table(name: &str, db_name: &str) -> Result<(), Error> {
    storage::table::drop_table(name, db_name)
}

/// # Describe Table
fn describe_table(name: &str, db_name: &str) -> Result<Vec<String>, Error> {
    storage::table::describe_table(name, db_name)
}