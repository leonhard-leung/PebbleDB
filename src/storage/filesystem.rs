use std::fs;
use crate::constants::system::ROOT;
use crate::constants::format::{MAGIC_NUMBER, FILE_EXTENSION, FILE_FORMAT_VERSION, HEADER_SIZE};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn list_databases() -> std::io::Result<Vec<String>> {
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
pub fn create_database(name: &str) -> std::io::Result<()> {
    // create path
    let path = create_path(name, FILE_EXTENSION);

    // create pdb file
    let mut file = File::create_new(path)?;

    // magic number
    file.write_all(MAGIC_NUMBER)?;

    // file version
    file.write_all(&[FILE_FORMAT_VERSION])?;

    // add reserve headers
    let bytes_written = MAGIC_NUMBER.len() + 1;
    let padding = HEADER_SIZE - bytes_written;
    file.write_all(&vec![0u8; padding])?;

    Ok(())
}

pub fn drop_database(name: &str) -> std::io::Result<()> {
    // obtain database list
    let path = create_path(name, FILE_EXTENSION);

    // remove database file
    fs::remove_file(path)?;

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