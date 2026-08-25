use crate::cli;
use crate::database::api;
use crate::database::model::Output;
use crate::runtime::session::Session;
use crate::shared::command::{DatabaseCommand, RecordCommand, TableCommand};
use crate::shared::error::Error;

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
            let table = cli::wizard::create_table_wizard(&table_name)?;
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

pub fn execute_record(
    command: RecordCommand,
    session: &mut Session
) -> Result<Output, Error> {
    Ok(Output::Message("Record Command".to_string()))
}