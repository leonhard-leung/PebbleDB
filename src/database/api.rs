//! # Database API
//! Provides the application-facing interface for database and table operations.
//! Performs validation before delegating the persistence operations to the storage layer.

use crate::database::model::{Column, DataType, Record, SerializedRecord, Table};
use crate::runtime::session::Session;
use crate::storage;
use crate::shared::error::{DatabaseError, Error, RecordError, TableError};

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
    db_name: &str
) -> Result<(), Error> {
    let list = storage::database::list_databases()?;

    if list.iter().any(| db | db.eq_ignore_ascii_case(db_name)) {
        return Err(Error::Database(DatabaseError::DatabaseAlreadyExists));
    }

    storage::database::create_database(db_name)
}

/// # drop_database
/// Drops a database through the storage layer.
pub fn drop_database(
    db_name: &str
) -> Result<(), Error> {
    let list = storage::database::list_databases()?;

    if !list.iter().any(| db | db.eq_ignore_ascii_case(db_name)) {
        return Err(Error::Database(DatabaseError::DatabaseNotFound));
    }

    storage::database::drop_database(db_name)
}

/// # use_database
/// Selects a database for the current session.
pub fn use_database(
    db_name: &str,
    session: &mut Session
) -> Result<(), Error> {
    if db_name.eq_ignore_ascii_case("none") {
        session.current_database = None;
        return Ok(());
    }

    let list = storage::database::list_databases()?;

    if list.iter().any(| db | db.eq_ignore_ascii_case(db_name)) {
        session.current_database = Some(db_name.to_string());
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
    data: Vec<String>,
    db_name: &str
) -> Result<(), Error>{
    let list = storage::table::list_tables(db_name)?;

    if list.iter().any(| t | t.eq_ignore_ascii_case(&data[0])) {
        return Err(Error::Table(TableError::TableAlreadyExists))
    }

    let mut columns: Vec<Column> = Vec::new();
    for index in (1..data.len() - 1).step_by(2) {
        let column = Column {
            name: data[index].to_owned(),
            data_type: DataType::from_str(&data[index + 1])
        };
        columns.push(column);
    }

    let table = Table {
        name: data[0].to_owned(),
        columns,
        row_count: 0
    };

    storage::table::create_table(table, db_name)
}

/// # drop_table
/// Drops a table through the storage layer.
pub fn drop_table(
    table_name: &str,
    db_name: &str
) -> Result<(), Error> {
    let list = storage::table::list_tables(db_name)?;

    if !list.iter().any(| t | t.eq_ignore_ascii_case(table_name)) {
        return Err(Error::Table(TableError::TableNotFound));
    }

    storage::table::drop_table(table_name, db_name)
}

/// # describe_table
/// Retrieves the metadata of the specified table.
///
/// ## Format:
/// - Index 0: Table Name
/// - Index 1: Column Count
/// - Index 2: Record Count
/// - Index 3 to Metadata Length: Column Name and Column Type (converted to string)
pub fn describe_table(
    table_name: &str,
    db_name: &str
) -> Result<Vec<String>, Error> {
    let list = storage::table::list_tables(db_name)?;

    if !list.iter().any(| t | t.eq_ignore_ascii_case(table_name)) {
        return Err(Error::Table(TableError::TableNotFound));
    }

    let mut metadata = storage::table::describe_table(table_name, db_name)?;

    // update data type id and convert to string description
    for index in (4..metadata.len()).step_by(2) {
        let column_type = &metadata[index].parse::<u8>().unwrap();
        metadata[index] = DataType::from_id(*column_type).to_string();
    }

    Ok(metadata)
}

// =================================================================================================
// Record
// =================================================================================================

/// # insert_record
/// Inserts a new record into the specified table.
pub fn insert_record(
    data: Vec<String>,
    table_name: &str,
    db_name: &str
) -> Result<(), Error> {
    let list = storage::table::list_tables(db_name)?;

    if !list.iter().any(| t | t.eq_ignore_ascii_case(table_name)) {
        return Err(Error::Table(TableError::TableNotFound));
    }

    let columns = storage::table::get_table_columns(table_name, db_name)?;
    if data.len() != columns.len() {
        return Err(Error::Record(RecordError::RecordLengthMismatch))
    }

    let cols = columns
        .iter()
        .map(|(name, id)| Column{ name: name.clone(), data_type: DataType::from_id(*id)})
        .collect::<Vec<Column>>();

    let mut error_message = String::new();
    for (index, (_, data_type_id)) in columns.iter().enumerate() {
        let data_type = DataType::from_id(*data_type_id);

        let (is_valid, err) = data_type.validate(&data[index]);

        if !is_valid {
            error_message.push_str(&format!(
                "  | {}: {}\n",
                columns[index].0,
                err
            ));
        }
    }
    if !error_message.is_empty() {
        return Err(Error::Record(RecordError::InvalidRecord(error_message)))
    }

    let record = Record { data, columns: cols };

    storage::record::insert_record(record.serialize()?, table_name, db_name)
}

pub fn select_record(
    id: u32,
    table_name: &str,
    db_name: &str
) -> Result<Record, Error> {
    let list = storage::table::list_tables(db_name)?;

    if !list.iter().any(| t | t.eq_ignore_ascii_case(table_name)) {
        return Err(Error::Table(TableError::TableNotFound));
    }

    let columns = storage::table::get_table_columns(table_name, db_name)?;
    let payload_size = columns.iter().map(| n | DataType::from_id(n.1).size()).sum::<usize>();

    let data = storage::record::read_record(
        id,
        payload_size,
        table_name,
        db_name
    )?;

    let serialized = SerializedRecord {
        data,
        payload_size
    };

    Ok(serialized.deserialize(&columns)?)
}

pub fn update_record(
    updated_data: Vec<String>,
    id: u32,
    table_name: &str,
    db_name: &str,
) -> Result<(), Error> {
    let list = storage::table::list_tables(db_name)?;

    if !list.iter().any(|t| t.eq_ignore_ascii_case(table_name)) {
        return Err(Error::Table(TableError::TableNotFound));
    }

    let columns = storage::table::get_table_columns(table_name, db_name)?;
    if updated_data.len() != columns.len() {
        return Err(Error::Record(RecordError::RecordLengthMismatch))
    }

    let cols = columns
        .iter()
        .map(|(name, id)| Column{ name: name.clone(), data_type: DataType::from_id(*id)})
        .collect::<Vec<Column>>();

    let mut error_message = String::new();

    for (index, col) in cols.iter().enumerate() {
        let data_type = &col.data_type;

        let (is_valid, err) = data_type.validate(&updated_data[index]);

        if !is_valid {
            error_message.push_str(&format!(
                "  | {}: {}\n",
                columns[index].0,
                err
            ));
        }
    }
    if !error_message.is_empty() {
        return Err(Error::Record(RecordError::InvalidRecord(error_message)))
    }

    let record = Record { data: updated_data, columns: cols };
    let serialized = record.serialize()?;

    storage::record::update_record(serialized.data, id, table_name, db_name)
}

pub fn delete_record(
    id: &u32,
    table_name: &str,
    db_name: &str,
) -> Result<(), Error> {
    let list = storage::table::list_tables(&db_name)?;

    if !list.iter().any(|t| t.eq_ignore_ascii_case(table_name)) {
        return Err(Error::Table(TableError::TableNotFound));
    }

    let columns = storage::table::get_table_columns(table_name, db_name)?;
    let payload_size = columns.iter().map(| n | DataType::from_id(n.1).size()).sum::<usize>();

    storage::record::delete_record(id, payload_size, table_name, db_name)
}