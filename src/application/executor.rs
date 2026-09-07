//! # Application Executor
//! Handles CLI command execution by coordinating the CLI, database API, and session state.

use crate::cli;
use crate::database::api;
use crate::runtime::session::Session;
use crate::shared::command::{DatabaseCommand, RecordCommand, TableCommand};
use crate::shared::error::Error;
use crate::shared::output::Output;

/// # execute_database
/// Executes a database command and updates the session when required.
pub fn execute_database(
    command: DatabaseCommand,
    session:&mut Session
) -> Result<Output, Error> {
    match command {
        DatabaseCommand::List => {
            let databases = api::list_databases()?;
            Ok(Output::DatabaseList(databases))
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
            Ok(Output::TablesList(tables))
        },
        TableCommand::Create(table_name) => {
            let data = cli::wizard::create_table_wizard(&table_name)?;
            api::create_table(data, &db_name)?;
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

/// # execute_record
/// Executes a record command within the database selected in the current session.
pub fn execute_record(
    command: RecordCommand,
    session: &mut Session
) -> Result<Output, Error> {
    let Some(db_name) = session.current_database.as_ref() else {
        return Err(Error::NoSelectedDatabase);
    };
    
    match command {
        RecordCommand::Insert(table_name) => {
            let metadata = api::describe_table(&table_name, &db_name)?;
            let columns = metadata
                .iter()
                .skip(3)
                .step_by(2)
                .cloned()
                .collect::<Vec<String>>();
            
            let data = cli::wizard::insert_record_wizard(columns)?;
            api::insert_record(data, &table_name, &db_name)?;

            Ok(Output::Message("Record added".to_string()))
        },
        RecordCommand::Select(table_name, id) => {
            let record = api::select_record(id, &table_name, &db_name)?;
            
            let output: Vec<(String, String)> = record
                .data
                .into_iter()
                .zip(record.columns.into_iter().map(|col| col.name))
                .collect();

            Ok(Output::Record(output))
        }
        _ => Ok(Output::Message("Record Command".to_string()))
    }
}