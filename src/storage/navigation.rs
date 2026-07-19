use crate::constants::format::{DATABASE_HEADER_SIZE, FILE_HEADER_SIZE, TABLE_BLOCK_SIZE};
use crate::types::error::Error;
use std::fs::File;
use std::io::{Seek, SeekFrom};

pub fn move_to_database_header(file: &mut File) -> Result<(), Error> {
    file.seek(SeekFrom::Start(FILE_HEADER_SIZE as u64))?;
    Ok(())
}

pub fn move_to_nth_table_block(file: &mut File, index: u32) -> Result<(), Error> {
    let offset = FILE_HEADER_SIZE 
        + DATABASE_HEADER_SIZE 
        + (index as usize * TABLE_BLOCK_SIZE);
    
    file.seek(SeekFrom::Start(offset as u64))?;
    Ok(())
}

pub fn table_offset(index: u32) -> u64 {
    (FILE_HEADER_SIZE + DATABASE_HEADER_SIZE + (index as usize * TABLE_BLOCK_SIZE)) as u64
}