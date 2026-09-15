use crate::constants::format::{COLUMN_COUNT_SIZE, COLUMN_DEFINITION_SIZE, COLUMN_NAME_SIZE, DATABASE_HEADER_SIZE, DATABASE_MAGIC_NUMBER_OFFSET, DATABASE_MAGIC_NUMBER_SIZE, FILE_HEADER_SIZE, RECORD_BLOCK_START_OFFSET, RECORD_EMPTY_MAGIC_NUMBER, RECORD_MAGIC_NUMBER, RECORD_MAGIC_NUMBER_SIZE, ROW_COUNT_SIZE, TABLE_BLOCK_SIZE, TABLE_BLOCK_START_OFFSET, TABLE_COUNT_SIZE, TABLE_MAGIC_NUMBER, TABLE_MAGIC_NUMBER_SIZE, TABLE_NAME_SIZE};
use crate::shared::error::Error;
use crate::storage::filesystem;
use std::fs::File;

// =================================================================================================
// Table Helper Functions
// =================================================================================================
pub fn read_table_count(
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

pub fn read_table_magic_number(
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

pub fn read_table_name(
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

pub fn read_column_count(
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

pub fn read_row_count(
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

pub fn read_column_definition(
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
    let data_type_id = u8::from_le_bytes(type_buf.try_into().unwrap());

    Ok((column_name.to_owned(), data_type_id))
}

pub fn get_table_index(
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

pub fn find_first_free_table(
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
        filesystem::move_current_file_cursor(
            file,
            TABLE_BLOCK_SIZE as i64 - TABLE_MAGIC_NUMBER_SIZE as i64
        )?;

        index += 1;
    }
}

// =================================================================================================
// Record Helper Functions
// =================================================================================================
pub fn find_first_free_record(
    file: &mut File,
    table_index: usize,
    payload_size: usize,
) -> std::io::Result<usize> {
    let record_size = RECORD_MAGIC_NUMBER_SIZE +
        size_of::<u32>() +
        payload_size;
    let offset = RECORD_BLOCK_START_OFFSET + (TABLE_BLOCK_SIZE * table_index) as u64;

    filesystem::move_file_cursor_at(file, offset)?;

    let mut index = 0;
    loop {
        let mut buffer = [0u8; RECORD_MAGIC_NUMBER_SIZE];

        match filesystem::read(file, &mut buffer) {
            Ok(()) => {
                if &buffer != RECORD_MAGIC_NUMBER {
                    return Ok(index);
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(index),
            Err(err) => return Err(err),
        }
        filesystem::move_current_file_cursor(
            file,
            (record_size - RECORD_MAGIC_NUMBER_SIZE) as i64
        )?;

        index += 1;
    }
}