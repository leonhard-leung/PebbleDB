use crate::constants::format::{COLUMN_DEFINITION_OFFSET, COLUMN_NAME_SIZE, DATABASE_HEADER_OFFSET, DATABASE_HEADER_SIZE, FILE_EXTENSION, FILE_HEADER_SIZE, TABLE_BLOCK_SIZE, TABLE_BLOCK_START_OFFSET, TABLE_COUNT_SIZE, TABLE_EMPTY_MAGIC_NUMBER, TABLE_MAGIC_NUMBER, TABLE_MAGIC_NUMBER_SIZE, TABLE_NAME_SIZE};
use crate::database::model::Table;
use crate::storage::filesystem;
use crate::types::error::Error;
use std::fs::File;

/// # List Tables
pub fn list_tables(db_name: &str) -> Result<Vec<String>, Error> {
    let mut tables: Vec<String> = Vec::new();

    // open file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // obtain table count (located in database header)
    let mut table_count_buf = [0u8; TABLE_COUNT_SIZE];
    filesystem::read_at(&mut file, DATABASE_HEADER_OFFSET, &mut table_count_buf)?;
    let table_count = u32::from_le_bytes(table_count_buf);

    // navigate to table block
    let mut index = 0;
    while tables.len() < table_count as usize {
        // get table magic number
        let mut magic_buf = [0u8; TABLE_MAGIC_NUMBER_SIZE];
        filesystem::read_at(&mut file,
                TABLE_BLOCK_START_OFFSET + (TABLE_BLOCK_SIZE * index) as u64,
                &mut magic_buf)?;

        // validate table magic number
        if &magic_buf == TABLE_MAGIC_NUMBER {
            // obtain table name
            let mut name_buf = [0u8; TABLE_NAME_SIZE];
            filesystem::read(&mut file, &mut name_buf)?;

            // push to vector
            tables.push(String::from_utf8_lossy(&name_buf).to_string());

            index += 1;
        } else {
            index += 1;
        }
    }
    Ok(tables)
}

/// # Create Table
pub fn create_table(table: Table, db_name: &str) -> Result<(), Error> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // obtain table count (located in database header)
    let mut table_count_buf = [0u8; TABLE_COUNT_SIZE];
    filesystem::read_at(&mut file, DATABASE_HEADER_OFFSET, &mut table_count_buf)?;
    let table_count = u32::from_le_bytes(table_count_buf);

    // write table magic number
    let first_free_table = find_first_free_table(&mut file)?;
    filesystem::write_at(&mut file,
             TABLE_BLOCK_START_OFFSET
                 + (TABLE_BLOCK_SIZE * first_free_table as usize) as u64,
             TABLE_MAGIC_NUMBER)?;

    // table name
    let name_buf = table.name.as_bytes();
    filesystem::write(&mut file, name_buf)?;
    filesystem::write_padding(&mut file, name_buf.len(), TABLE_NAME_SIZE)?;

    // column count
    let column_count = table.columns.len() as u32;
    filesystem::write(&mut file, &column_count.to_le_bytes())?;

    // row count
    filesystem::write(&mut file, &0u32.to_le_bytes())?;

    // COLUMN DEFINITIONS: Column Name, Column Type
    filesystem::move_file_cursor_at(&mut file,
                        COLUMN_DEFINITION_OFFSET +
                (TABLE_BLOCK_SIZE * first_free_table as usize) as u64)?;

    for col in table.columns {
        // column name
        let col_name_buf = col.name.as_bytes();
        filesystem::write(&mut file, col_name_buf)?;
        filesystem::write_padding(&mut file, col_name_buf.len(), COLUMN_NAME_SIZE)?;

        // column type
        filesystem::write(&mut file, &col.data_type.id().to_le_bytes())?;
    }

    // update table number in the database header by adding plus 1 if successful
    filesystem::write_at(&mut file, DATABASE_HEADER_OFFSET, &(table_count + 1).to_le_bytes())?;

    Ok(())
}

pub fn drop_table(name: &str, db_name: &str) -> Result<(), Error> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // loop through tables
    let mut index = 0;
    loop {
        // check if magic number is correct
        let mut magic_buf = [0u8; TABLE_MAGIC_NUMBER_SIZE];
        filesystem::read_at(&mut file,
                TABLE_BLOCK_START_OFFSET + (TABLE_BLOCK_SIZE * index) as u64,
                &mut magic_buf)?;

        if &magic_buf == TABLE_MAGIC_NUMBER {
            // check if name matches with the target
            let mut name_buffer = [0u8; TABLE_NAME_SIZE];
            filesystem::read(&mut file, &mut name_buffer)?;

            let stored_name = std::str::from_utf8(&name_buffer)?
                .trim_end_matches('\0');

            if stored_name == name {
                // change magic number to empty magic number
                filesystem::write_at(&mut file,
                         TABLE_BLOCK_START_OFFSET + (TABLE_BLOCK_SIZE * index) as u64,
                         TABLE_EMPTY_MAGIC_NUMBER)?;

                // get current table count
                let mut table_count_buf = [0u8; TABLE_COUNT_SIZE];
                filesystem::read_at(&mut file, DATABASE_HEADER_OFFSET, &mut table_count_buf)?;
                let table_count = u32::from_le_bytes(table_count_buf);

                // update table count
                filesystem::write_at(&mut file, DATABASE_HEADER_OFFSET, &(table_count - 1).to_le_bytes())?;

                break;
            }
        }
        index += 1;
    }
    Ok(())
}

pub fn describe_table(table_name: &str, db_name: &str) -> std::io::Result<()> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // search table
    // move_to_database_header(&mut file)?;
    // loop {
    //     let mut buffer = [0u8; TABLE_MAGIC_SIZE];
    //
    // }

    Ok(())
}

// =================================================================================================
// Helper Function
// =================================================================================================
fn find_first_free_table(file: &mut File) -> std::io::Result<u32> {
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