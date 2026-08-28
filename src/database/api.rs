use crate::database::model::Table;
use crate::runtime::session::Session;
use crate::storage;
use crate::shared::error::{DatabaseError, Error, TableError};

// =================================================================================================
// Database
// =================================================================================================

/// # list_databases
/// Retrieves the databases available in the storage layer.
pub fn list_databases() -> Result<Vec<String>, Error> {
    storage::database::list_databases()
}

/// # create_database
/// Creates a database through the storage layer.
pub fn create_database(
    name: &str
) -> Result<(), Error> {
    let list = storage::database::list_databases()?;

    if list.iter().any(| db | db.eq_ignore_ascii_case(name)) {
        return Err(Error::Database(DatabaseError::DatabaseAlreadyExists));
    }

    storage::database::create_database(name)
}

/// # drop_database
/// Drops a database through the storage layer.
pub fn drop_database(
    name: &str
) -> Result<(), Error> {
    let list = storage::database::list_databases()?;

    if !list.iter().any(| db | db.eq_ignore_ascii_case(name)) {
        return Err(Error::Database(DatabaseError::DatabaseNotFound));
    }

    storage::database::drop_database(name)
}

/// # use_database
/// Selects a database for the current session.
pub fn use_database(
    name: &str,
    session: &mut Session
) -> Result<(), Error> {
    if name.eq_ignore_ascii_case("none") {
        session.current_database = None;
        return Ok(());
    }

    let list = storage::database::list_databases()?;

    if list.iter().any(| db | db.eq_ignore_ascii_case(name)) {
        session.current_database = Some(name.to_string());
        return Ok(());
    }
    Err(Error::Database(DatabaseError::DatabaseNotFound))
}

// =================================================================================================
// Table
// =================================================================================================

/// # list_tables
/// Retrieves the active tables from the specified database.
pub fn list_tables(
    db_name: &str
) -> Result<Vec<String>, Error> {
    storage::table::list_tables(db_name)
}

/// # create_table
/// Creates a table through the storage layer.
pub fn create_table(
    table: Table,
    db_name: &str
) -> Result<(), Error>{
    let list = storage::table::list_tables(db_name)?;

    if list.iter().any(| t | t.eq_ignore_ascii_case(&table.name)) {
        return Err(Error::Table(TableError::TableAlreadyExists))
    }

    storage::table::create_table(table, db_name)
}

/// # drop_table
/// Drops a table through the storage layer.
pub fn drop_table(
    name: &str,
    db_name: &str
) -> Result<(), Error> {
    let list = storage::table::list_tables(db_name)?;

    if !list.iter().any(| t | t.eq_ignore_ascii_case(name)) {
        return Err(Error::Table(TableError::TableNotFound));
    }

    storage::table::drop_table(name, db_name)
}

/// # describe_table
/// Retrieves the metadata of the specified table.
pub fn describe_table(
    name: &str,
    db_name: &str
) -> Result<Vec<String>, Error> {
    let list = storage::table::list_tables(db_name)?;

    if !list.iter().any(| t | t.eq_ignore_ascii_case(name)) {
        return Err(Error::Table(TableError::TableNotFound));
    }

    storage::table::describe_table(name, db_name)
}