//! # Database
//!

use crate::constants::format::{DATABASE_MAGIC_NUMBER, FILE_EXTENSION, FILE_FORMAT_VERSION, FILE_MAGIC_NUMBER};
use crate::constants::system::ROOT;
use crate::storage::filesystem;
use crate::shared::error::Error;
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

    // FILE HEADER: Magic Number, File Version
    filesystem::write(&mut file, FILE_MAGIC_NUMBER)?;
    filesystem::write(&mut file, &[FILE_FORMAT_VERSION])?;

    // DATABASE METADATA: Magic Number, Table Count
    filesystem::write(&mut file, DATABASE_MAGIC_NUMBER)?;
    filesystem::write(&mut file, &0u8.to_le_bytes())?;

    Ok(())
}

/// # Drop Database
pub fn drop_database(name: &str) -> Result<(), Error> {
    // get database list
    let path = filesystem::create_file_path(name, FILE_EXTENSION);

    // remove database file
    fs::remove_file(path)?;

    Ok(())
}