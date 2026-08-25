use crate::database::model::Table;
use crate::runtime::session::Session;
use crate::storage;
use crate::types::error::Error;

/// # list_databases
/// Retrieves the databases available in the storage layer.
pub fn list_databases() -> Result<Vec<String>, Error> {
    storage::database::list_databases()
}

/// # create_database
/// Creates a database through the storage layer.
pub fn create_database(name: &str) -> Result<(), Error> {
    storage::database::create_database(name)
}

/// # drop_database
/// Drops a database through the storage layer.
pub fn drop_database(name: &str) -> Result<(), Error> {
    storage::database::drop_database(name)
}

/// # use_database
/// Selects a database for the current session.
pub fn use_database(name: &str, session: &mut Session) -> Result<(), Error> {
    if name == "none" {
        session.current_database = None;

    }

    let list = storage::database::list_databases()?;
    if list.iter().any(| db | db.eq_ignore_ascii_case(name)) {
        session.current_database = Some(name.to_string());
    }
    Ok(())
}

/// # list_tables
/// Retrieves the active tables from the specified database.
pub fn list_tables(db_name: &str) -> Result<Vec<String>, Error> {
    storage::table::list_tables(db_name)
}

/// # create_table
/// Creates a table through the storage layer.
pub fn create_table(table: Table, db_name: &str) -> Result<(), Error>{
    storage::table::create_table(table, db_name)
}

/// # drop_table
/// Drops a table through the storage layer.
pub fn drop_table(name: &str, db_name: &str) -> Result<(), Error> {
    storage::table::drop_table(name, db_name)
}

/// # describe_table
/// Retrieves the metadata of the specified table.
pub fn describe_table(name: &str, db_name: &str) -> Result<Vec<String>, Error> {
    storage::table::describe_table(name, db_name)
}