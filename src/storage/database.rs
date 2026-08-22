//! # Database
//!

use crate::constants::format::{DATABASE_HEADER_SIZE, FILE_EXTENSION, FILE_FORMAT_VERSION, FILE_HEADER_SIZE, FILE_MAGIC_NUMBER, TABLE_COUNT_SIZE};
use crate::constants::system::ROOT;
use crate::storage::filesystem;
use crate::types::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::storage::filesystem::list_files;

// =================================================================================================
// Database Functions
// =================================================================================================

/// # List Databases
pub fn list_databases() -> Result<Vec<String>, Error> {
    let mut databases = list_files(Path::new(&ROOT))?;

    databases.retain(|file| file.ends_with(FILE_EXTENSION));

    databases = databases
        .into_iter()
        .filter_map(|file| {
            Path::new(&file)
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        }).collect();

    Ok(databases)
}

/// # Create Database
pub fn create_database(name: &str) -> Result<(), Error> {
    // create path
    let path = filesystem::create_file_path(name, FILE_EXTENSION);

    // create pdb file
    let mut file = File::create_new(path)?;

    // HEADER: Magic Number, File Version
    file.write_all(FILE_MAGIC_NUMBER)?;
    file.write_all(&[FILE_FORMAT_VERSION])?;
    filesystem::write_padding(&mut file, FILE_MAGIC_NUMBER.len() + 1, FILE_HEADER_SIZE)?;

    // DATABASE METADATA: Table Count
    file.write_all(&0u32.to_le_bytes())?;
    filesystem::write_padding(&mut file, TABLE_COUNT_SIZE, DATABASE_HEADER_SIZE)?;

    Ok(())
}

/// # Drop Database
pub fn drop_database(name: &str) -> Result<(), Error> {
    // obtain database list
    let path = filesystem::create_file_path(name, FILE_EXTENSION);

    // remove database file
    fs::remove_file(path)?;

    Ok(())
}