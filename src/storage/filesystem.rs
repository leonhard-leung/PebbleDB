//! # Filesystem
//! Provides low-level file and I/O operations for the database storage layer.

use crate::constants::system::ROOT;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

// =================================================================================================
// File Operations
// =================================================================================================

/// # create_file_path
/// Creates a `PathBuf` using the given filename and file extension, attached to the `ROOT`
/// constant.
///
/// ## Example
/// ```rust
/// // Assuming ROOT is from a .env or another rs file
/// pub const ROOT: &str = r"C:\Some\Path";
///
/// let path = create_file_path("rustdoc", "doc");
/// ```
pub fn create_file_path(filename: &str, file_extension: &str) -> PathBuf {
    let mut path = PathBuf::from(ROOT).join(filename);
    path.set_extension(file_extension);
    path
}

/// # open_file
/// Opens a file with read and write privileges.
///
/// ## Example
/// ```rust
/// let path = Path::new(r"C:\Some\File.txt");
/// let mut file = open_file(path)?;
/// ```
pub fn open_file(path: &Path) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
}

/// # delete_file
/// Removes the target file given the path of the file.
///
/// ## Example
/// ```rust
/// let path = Path::new(r"C:\Some\File.txt");
/// delete_file(path)?;
/// ```
pub fn delete_file(path: &Path) -> std::io::Result<()> {
    std::fs::remove_file(path)
}

/// # list_files
/// Creates a vector containing the filenames and their respective extensions from the given
/// directory.
///
/// ## Example
/// ```rust
/// let path = Path::new(r"C:\Some\Directory");
/// let files = list_files(path)?;
/// ```
pub fn list_files(path: &Path) -> std::io::Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let path = entry?.path();

        if let Some(filename) = path.file_name() {
            files.push(filename.to_string_lossy().into_owned());
        }
    }
    Ok(files)
}

/// # read
/// Reads bytes from the current position of the file into the provided buffer. The file pointer
/// is moved by the number of bytes read.
///
/// ## Example
/// ```rust
/// let path = Path::new(r"C:\Some\File.txt");
/// let mut file = open_file(path)?;
/// let mut buf = [0u8; 20];
///
/// read(&mut file, &mut buf)?;
/// ```
pub fn read(file: &mut File, buf: &mut [u8]) -> std::io::Result<()> {
    file.read_exact(buf)?;
    Ok(())
}

/// # write
/// Writes bytes from the provided buffer to the current position of the file. The file pointer
/// is moved by the number of bytes written.
///
/// ## Example
/// ```rust
/// let path = Path::new(r"C:\Some\File.txt");
/// let mut file = open_file(path)?;
/// let buf = b"Hello World";
///
/// write(&mut file, buf)?;
/// ```
pub fn write(file: &mut File, buf: &[u8]) -> std::io::Result<()>{
    file.write_all(buf)?;
    Ok(())
}

/// # read_at
/// Reads bytes from the specified offset of the file into the provided buffer. The offset is
/// calculated from the start of the file.
///
/// ## Example
/// ```rust
/// let path = Path::new(r"C:\Some\File.txt");
/// let mut file = open_file(path)?;
/// let offset = 128;
/// let mut buf = [0u8; 20];
///
/// read_at(&mut file, offset, &mut buf)?;
/// ```
pub fn read_at(file: &mut File, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(buf)?;
    Ok(())
}

/// # write_at
/// Writes bytes from the buffer to the specified offset of the file. The offset is calculated
/// from the start of the file.
///
/// ## Example
/// ```rust
/// let path = Path::new(r"C:\Some\File.txt");
/// let mut file = open_file(path)?;
/// let offset = 128;
/// let buf = b"Hello World";
///
/// write_at(&mut file, offset, buf)?;
/// ```
pub fn write_at(file: &mut File, offset: u64, buf: &[u8]) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(buf)?;
    Ok(())
}

/// # write_padding
/// Adds zero byte padding to the current file pointer until the specified region size is reached.
/// The amount of padding is calculated from the number of bytes already written.
///
/// ## Example
/// ```rust
/// let path = Path::new(r"C:\Some\File.txt");
/// let mut file = open_file(path)?;
/// let value = b"Hello World";
///
/// file.write_all(value)?;
/// write_padding(&mut file, value.len(), 20)?;
/// ```
pub fn write_padding(file: &mut File, bytes_written:usize, region_size: usize) -> std::io::Result<()> {
    debug_assert!(bytes_written <= region_size); //TODO: Remove eventually

    let padding = region_size - bytes_written;
    file.write_all(&vec![0u8; padding])?;
    Ok(())
}

// =================================================================================================
// File Navigations
// =================================================================================================

/// # move_file_cursor
/// Moves the file pointer from the start of the file to the specified offset.
pub fn move_file_cursor_at(file: &mut File, offset: u64) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(offset))?;
    Ok(())
}

/// # move_current_file_cursor
/// Moves the current file pointer to the specified offset.
pub fn move_current_file_cursor(file: &mut File, offset: i64) -> std::io::Result<()> {
    file.seek(SeekFrom::Current(offset))?;
    Ok(())
}