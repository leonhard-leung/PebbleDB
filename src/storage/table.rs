//! # Table
//! Provides operations for creating, listing, describing, and dropping tables.

use crate::constants::format::{COLUMN_COUNT_SIZE, COLUMN_DEFINITION_OFFSET, COLUMN_DEFINITION_SIZE, COLUMN_NAME_SIZE, DATABASE_HEADER_OFFSET, DATABASE_HEADER_SIZE, DATABASE_MAGIC_NUMBER_OFFSET, DATABASE_MAGIC_NUMBER_SIZE, FILE_EXTENSION, FILE_HEADER_SIZE, ROW_COUNT_SIZE, TABLE_BLOCK_SIZE, TABLE_BLOCK_START_OFFSET, TABLE_COUNT_SIZE, TABLE_EMPTY_MAGIC_NUMBER, TABLE_MAGIC_NUMBER, TABLE_MAGIC_NUMBER_SIZE, TABLE_NAME_SIZE};
use crate::database::model::Table;
use crate::storage::filesystem;
use crate::shared::error::Error;
use std::fs::File;

/// # list_tables
/// Lists all active tables in the specified database.
pub fn list_tables(
    db_name: &str
) -> Result<Vec<String>, Error> {
    // open file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // vector for storing table names
    let mut tables: Vec<String> = Vec::new();

    // get table count
    let table_count = read_table_count(&mut file)?;

    // navigate to table block
    let mut index = 0;
    while tables.len() < table_count as usize {
        // get table magic number
        let magic_number_buf = read_table_magic_number(&mut file, index)?;

        // check if magic number is correct
        if &magic_number_buf == TABLE_MAGIC_NUMBER {
            // get table name
            let table_name = read_table_name(&mut file, index)?;

            // push to vector
            tables.push(table_name);

            index += 1;
        } else {
            index += 1;
        }
    }
    Ok(tables)
}

/// # create_table
/// Creates a new table in the specified database and stores its metadata in the first
/// available table block.
pub fn create_table(
    table: Table,
    db_name: &str
) -> Result<(), Error> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // get table count
    let table_count = read_table_count(&mut file)?;

    // write table magic number
    let index = find_first_free_table(&mut file)?;
    filesystem::write_at(
        &mut file,
        TABLE_BLOCK_START_OFFSET + (TABLE_BLOCK_SIZE * index) as u64,
        TABLE_MAGIC_NUMBER,
    )?;

    // table name
    let name_bytes = table.name.as_bytes();
    let mut name_buf = [0u8; TABLE_NAME_SIZE];
    name_buf[..name_bytes.len()].copy_from_slice(name_bytes);
    filesystem::write(&mut file, &name_buf)?;

    // column count
    let column_count = table.columns.len() as u8;
    filesystem::write(&mut file, &column_count.to_le_bytes())?;

    // row count
    filesystem::write(&mut file, &0u32.to_le_bytes())?;

    // COLUMN DEFINITIONS: Column Name, Column Type
    for col in table.columns {
        // column name
        let col_name_bytes = col.name.as_bytes();
        let mut col_name_buf = [0u8; COLUMN_NAME_SIZE];
        col_name_buf[..col_name_bytes.len()].copy_from_slice(col_name_bytes);
        filesystem::write(&mut file, &col_name_buf)?;

        // column type
        filesystem::write(&mut file, &col.data_type.id().to_le_bytes())?;
    }

    // update table number in the database header by adding plus 1 if successful
    filesystem::write_at(
        &mut file,
        DATABASE_HEADER_OFFSET + DATABASE_MAGIC_NUMBER_SIZE as u64,
        &(table_count + 1).to_le_bytes()
    )?;

    Ok(())
}

/// # drop_table
/// Marks the specified table as inactive and removes it from the database's active
/// table count.
pub fn drop_table(
    table_name: &str,
    db_name: &str
) -> Result<(), Error> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // loop through tables
    let mut index = 0;
    loop {
        // get table magic number
        let magic_number_buf = read_table_magic_number(&mut file, index)?;

        // check if magic number is correct
        if &magic_number_buf == TABLE_MAGIC_NUMBER {
            // get table name
            let stored_name = read_table_name(&mut file, index)?;

            // check if name matches with the target
            if stored_name == table_name {
                // change magic number to empty magic number
                filesystem::write_at(&mut file,
                         TABLE_BLOCK_START_OFFSET + (TABLE_BLOCK_SIZE * index) as u64,
                         TABLE_EMPTY_MAGIC_NUMBER)?;

                // get current table count
                let table_count = read_table_count(&mut file)?;

                // update table count
                filesystem::write_at(
                    &mut file,
                    DATABASE_HEADER_OFFSET + DATABASE_MAGIC_NUMBER_SIZE as u64,
                    &(table_count - 1).to_le_bytes()
                )?;

                break;
            }
        }
        index += 1;
    }
    Ok(())
}

/// # describe_table
/// Returns the metadata of the specified table, including its name, column count, row count,
/// and column definitions.
pub fn describe_table(
    table_name: &str,
    db_name: &str
) -> Result<Vec<String>, Error> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // vec to store data
    let mut data: Vec<String> = Vec::new();

    // obtain table index
    let index = get_table_index(&mut file, table_name)?;

    // get table name
    let table_name = read_table_name(&mut file, index)?;
    data.push(table_name.to_string());

    // get column count
    let column_count = read_column_count(&mut file, index)?;
    data.push(column_count.to_string());

    // get record count
    let record_count = read_row_count(&mut file, index)?;
    data.push(record_count.to_string());

    // get columns and data type
    for i in 0..column_count {
        let offset = COLUMN_DEFINITION_OFFSET +
            (TABLE_BLOCK_SIZE * index) as u64 +
            (COLUMN_DEFINITION_SIZE * i as usize) as u64;

        let (column_name, column_type) = read_column_definition(
            &mut file,
            offset
        )?;

        data.push(column_name.to_string());
        data.push(column_type.to_string());
    }
    Ok(data)
}

// =================================================================================================
// Helper Function
// =================================================================================================
fn read_table_count(
    file: &mut File
) -> Result<u8, Error> {
    let offset = DATABASE_MAGIC_NUMBER_OFFSET + DATABASE_MAGIC_NUMBER_SIZE as u64;

    let mut buf = [0u8; TABLE_COUNT_SIZE];
    filesystem::read_at(
        file,
        offset,
        &mut buf
    )?;

    Ok(u8::from_le_bytes(buf))
}

fn read_table_magic_number(
    file: &mut File,
    index: usize
) -> Result<[u8; TABLE_MAGIC_NUMBER_SIZE], Error> {
    let offset = TABLE_BLOCK_START_OFFSET + (TABLE_BLOCK_SIZE * index) as u64;

    let mut buf = [0u8; TABLE_MAGIC_NUMBER_SIZE];
    filesystem::read_at(
        file,
        offset,
        &mut buf
    )?;

    Ok(buf)
}

fn read_table_name(
    file: &mut File,
    index: usize
) -> Result<String, Error> {
    let offset = TABLE_BLOCK_START_OFFSET +
        (TABLE_BLOCK_SIZE * index) as u64 +
        TABLE_MAGIC_NUMBER_SIZE as u64;

    let mut buf = [0u8; TABLE_NAME_SIZE];
    filesystem::read_at(
        file,
        offset,
        &mut buf
    )?;

    Ok(std::str::from_utf8(&buf)?.trim_end_matches('\0').to_owned())
}

fn read_column_count(
    file: &mut File,
    index: usize
) -> Result<u8, Error> {
    let offset = TABLE_BLOCK_START_OFFSET +
        (TABLE_BLOCK_SIZE * index) as u64 +
        TABLE_MAGIC_NUMBER_SIZE as u64 +
        TABLE_NAME_SIZE as u64;

    let mut buf = [0u8; COLUMN_COUNT_SIZE];
    filesystem::read_at(
        file,
        offset,
        &mut buf
    )?;

    Ok(u8::from_le_bytes(buf))
}

fn read_row_count(
    file: &mut File,
    index: usize
) -> Result<u32, Error> {
    let offset = TABLE_BLOCK_START_OFFSET +
        (TABLE_BLOCK_SIZE * index) as u64 +
        TABLE_MAGIC_NUMBER_SIZE as u64 +
        TABLE_NAME_SIZE as u64 +
        COLUMN_COUNT_SIZE as u64;

    let mut buf = [0u8; ROW_COUNT_SIZE];
    filesystem::read_at(
       file,
       offset,
       &mut buf
    )?;

    Ok(u32::from_le_bytes(buf))
}

fn read_column_definition(
    file: &mut File,
    offset: u64
) -> Result<(String, u8), Error> {
    let mut buf = [0u8; COLUMN_DEFINITION_SIZE];

    filesystem::read_at(
        file,
        offset,
        &mut buf
    )?;

    let (name_buf, type_buf) = buf.split_at(COLUMN_NAME_SIZE);

    let column_name = std::str::from_utf8(name_buf)?.trim_end_matches('\0');
    let column_type = u8::from_le_bytes(type_buf.try_into().unwrap());

    Ok((column_name.to_owned(), column_type))
}

fn get_table_index(
    file: &mut File,
    target: &str
) -> Result<usize, Error> {
    let mut index = 0;

    loop {
        // get magic number
        let magic_number_buf = read_table_magic_number(file, index)?;

        // check if magic number is correct
        if &magic_number_buf == TABLE_MAGIC_NUMBER {
            // get table name
            let stored_name = read_table_name(file, index)?;

            // check if acquired name matches target
            if stored_name.eq_ignore_ascii_case(target) {
                break;
            }
        }
        index += 1;
    }

    Ok(index)
}

fn find_first_free_table(
    file: &mut File
) -> std::io::Result<usize> {
    let offset = FILE_HEADER_SIZE + DATABASE_HEADER_SIZE;
    filesystem::move_file_cursor_at(file, offset as u64)?;

    let mut index = 0;
    loop {
        let mut buffer = [0u8; TABLE_MAGIC_NUMBER_SIZE];

        match filesystem::read(file, &mut buffer) {
            Ok(()) => {
                if &buffer != TABLE_MAGIC_NUMBER {
                    return Ok(index);
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(index),
            Err(err) => return Err(err),
        }
        filesystem::move_current_file_cursor(file,
                                             TABLE_BLOCK_SIZE as i64 -
                                                 TABLE_MAGIC_NUMBER_SIZE as i64)?;

        index += 1;
    }
}