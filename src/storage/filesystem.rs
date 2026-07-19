use std::fs;
use crate::constants::system::ROOT;
use crate::constants::format::{COLUMN_COUNT_SIZE, COLUMN_DEFINITION_SIZE, COLUMN_NAME_SIZE, FILE_EXTENSION, FILE_FORMAT_VERSION, FILE_HEADER_SIZE, FILE_MAGIC_NUMBER, FIRST_FREE_ROW_SIZE, ROW_COUNT_SIZE, TABLE_COUNT_SIZE, TABLE_MAGIC_NUMBER, TABLE_MAGIC_SIZE, TABLE_NAME_SIZE};
use crate::constants::format::{DATABASE_HEADER_SIZE, TABLE_BLOCK_SIZE, TABLE_HEADER_SIZE};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use crate::database::model::Table;
use crate::storage::navigation;
use crate::storage::navigation::{move_to_database_header, move_to_nth_table_block};
use crate::types::error::Error;

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # DATABASE SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # List Databases
pub fn list_databases() -> Result<Vec<String>, Error> {
    let mut databases: Vec<String> = Vec::new();

    // iterate through every file in the ROOT directory
    for entry in fs::read_dir(ROOT)? {
        let entry = entry?;
        let path = entry.path();

        // validate extension
        if path.extension() == Some(std::ffi::OsStr::new(FILE_EXTENSION)) {
            if let Some(file_name) = path.file_stem() {
                databases.push(file_name.to_string_lossy().into_owned());
            }
        }
    }

    Ok(databases)
}

/// # Create Database
pub fn create_database(name: &str) -> Result<(), Error> {
    // create path
    let path = create_path(name, FILE_EXTENSION);

    // create pdb file
    let mut file = File::create_new(path)?;

    // HEADER: Magic Number, File Version
    file.write_all(FILE_MAGIC_NUMBER)?;
    file.write_all(&[FILE_FORMAT_VERSION])?;
    write_padding(&mut file, FILE_MAGIC_NUMBER.len() + 1, FILE_HEADER_SIZE)?;

    // DATABASE METADATA: Table Count
    file.write_all(&0u32.to_le_bytes())?;
    write_padding(&mut file, TABLE_COUNT_SIZE, DATABASE_HEADER_SIZE)?;

    Ok(())
}

/// # Drop Database
pub fn drop_database(name: &str) -> Result<(), Error> {
    // obtain database list
    let path = create_path(name, FILE_EXTENSION);

    // remove database file
    fs::remove_file(path)?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # TABLE SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # List Tables
pub fn list_tables(db_name: &str) -> Result<Vec<String>, Error> {
    let mut tables: Vec<String> = Vec::new();

    // open file
    let path = create_path(db_name, FILE_EXTENSION);
    let mut file = open_db_file(&path)?;

    // obtain table count
    let mut table_buffer = [0u8; TABLE_COUNT_SIZE];
    move_to_database_header(&mut file)?;
    file.read_exact(&mut table_buffer)?;
    let table_count = u32::from_le_bytes(table_buffer);

    // navigate to table block
    let mut index = 0;
    while tables.len() < table_count as usize {
        move_to_nth_table_block(&mut file, index)?;

        // get table magic number
        let mut magic_buffer = [0u8; TABLE_MAGIC_SIZE];
        file.read_exact(&mut magic_buffer)?;

        // validate table magic number
        if &magic_buffer == TABLE_MAGIC_NUMBER {
            // obtain table name
            let mut name_buffer = [0u8; TABLE_NAME_SIZE];
            file.read_exact(&mut name_buffer)?;

            // push to vector
            (tables).push(String::from_utf8_lossy(&name_buffer).to_string());

            index += 1;
        } else {
            index += 1;
        }
    }

    Ok(tables)
}

/// # Create Table
pub fn create_table(table: Table, db_name: &str) -> std::io::Result<()> {
    // open .pdb file
    let path = create_path(db_name, FILE_EXTENSION);
    let mut file = open_db_file(&path)?;

    // move to database metadata
    seek_db_header(&mut file)?;

    // get table count
    let mut buffer = [0u8; TABLE_COUNT_SIZE];
    file.read_exact(&mut buffer)?;
    let table_count = u32::from_le_bytes(buffer);

    // TABLE HEADER: Table Magic Number, Table Name, Column Count, Row Count, First Free Row
    let first_free_table = find_first_free_table(&mut file)?;
    file.seek(SeekFrom::Start(navigation::table_offset(first_free_table)))?;

    // table magic number
    file.write_all(TABLE_MAGIC_NUMBER)?;

    // table name
    write_fixed_string(&mut file, &table.name, TABLE_NAME_SIZE)?;

    // column count
    let column_count = table.columns.len() as u32;
    file.write_all(&column_count.to_le_bytes())?;

    // row count
    file.write_all(&0u32.to_le_bytes())?;

    // first free row
    file.write_all(&0u32.to_le_bytes())?;
    write_padding(&mut file, TABLE_MAGIC_SIZE +
        TABLE_NAME_SIZE + COLUMN_COUNT_SIZE +
        ROW_COUNT_SIZE + FIRST_FREE_ROW_SIZE, TABLE_HEADER_SIZE)?;

    // COLUMN DEFINITIONS: Column Name, Column Type
    for col in table.columns {
        // column name
        write_fixed_string(&mut file, &col.name, COLUMN_NAME_SIZE)?;

        // column type
        file.write_all(&col.data_type.id().to_le_bytes())?;
    }

    // reserve space for row entries
    let bytes_written = TABLE_HEADER_SIZE + (column_count as usize * COLUMN_DEFINITION_SIZE);
    write_padding(&mut file, bytes_written, TABLE_BLOCK_SIZE)?;

    // update table number in the database header by adding plus 1 if successful
    seek_db_header(&mut file)?;
    file.write_all(&(table_count + 1).to_le_bytes())?;
    Ok(())
}

pub fn describe_table(table_name: &str, db_name: &str) -> std::io::Result<()> {
    // open .pdb file
    let path = create_path(db_name, FILE_EXTENSION);
    let mut file = open_db_file(&path)?;

    // search table
    seek_db_header(&mut file)?;
    loop {
        let mut buffer = [0u8; TABLE_MAGIC_SIZE];

    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # HELPER FUNCTION
///////////////////////////////////////////////////////////////////////////////////////////////////

fn create_path(file_name: &str, file_extension: &str) -> PathBuf {
    let mut path: PathBuf = PathBuf::from(ROOT).join(file_name);
    path.set_extension(file_extension);
    path
}

fn open_db_file(path: &PathBuf) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
}

fn write_padding(file: &mut File, bytes_written:usize, region_size: usize) -> std::io::Result<()> {
    debug_assert!(bytes_written <= region_size);

    let padding = region_size - bytes_written;
    file.write_all(&vec![0u8; padding])?;
    Ok(())
}

fn write_fixed_string(file: &mut File, string: &str, size: usize) -> std::io::Result<()> {
    let bytes = string.as_bytes();
    debug_assert!(bytes.len() <= size);

    let mut buffer = vec![0u8; size];
    buffer[..bytes.len()].copy_from_slice(bytes);
    file.write_all(&buffer)?;
    Ok(())
}

fn find_first_free_table(file: &mut File) -> std::io::Result<u32> {
    let offset = FILE_HEADER_SIZE + DATABASE_HEADER_SIZE;
    file.seek(SeekFrom::Start(offset as u64))?;

    let mut index = 0;
    loop {
        let mut buffer = [0u8; TABLE_MAGIC_SIZE];

        match file.read_exact(&mut buffer) {
            Ok(()) => {
                if &buffer != TABLE_MAGIC_NUMBER {
                    return Ok(index);
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(index),
            Err(err) => return Err(err),
        }

        file.seek(SeekFrom::Current(TABLE_BLOCK_SIZE as i64 - TABLE_MAGIC_SIZE as i64))?;

        index += 1;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # HELPER FUNCTION (FILE NAVIGATION)
///////////////////////////////////////////////////////////////////////////////////////////////////
fn seek_db_header(file: &mut File) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(FILE_HEADER_SIZE as u64))?;
    Ok(())
}

// DEV NOTES:
// v0.1 Scan every table page
// v0.2 Maintain first free table
// v0.3 Maintain linked free list
// v0.4 Bitmap allocator?