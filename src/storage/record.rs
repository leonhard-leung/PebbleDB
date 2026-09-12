//! # Record
//! Provides operations for selecting, inserting, deleting, and updating records.

use crate::constants::format::{COLUMN_COUNT_SIZE, FILE_EXTENSION, RECORD_BLOCK_START_OFFSET, RECORD_EMPTY_MAGIC_NUMBER, RECORD_MAGIC_NUMBER, RECORD_MAGIC_NUMBER_SIZE, TABLE_BLOCK_SIZE, TABLE_BLOCK_START_OFFSET, TABLE_MAGIC_NUMBER_SIZE, TABLE_NAME_SIZE};
use crate::database::model::SerializedRecord;
use crate::shared::error::{Error, RecordError};
use crate::storage::{filesystem, util};

pub fn insert_record(
    record: SerializedRecord,
    table_name: &str,
    db_name: &str,
) -> Result<(), Error> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // get table index
    let table_index = util::get_table_index(&mut file, table_name)?;

    // get record index
    let record_index = util::find_first_free_record(&mut file, table_index, record.payload_size)?;

    // calculate offset
    let record_size = RECORD_MAGIC_NUMBER_SIZE +
        size_of::<u32>() +
        record.payload_size;
    let offset = RECORD_BLOCK_START_OFFSET +
        (TABLE_BLOCK_SIZE * table_index) as u64 +
        (record_size * record_index) as u64;

    // write record magic number
    filesystem::write_at(
        &mut file,
        offset,
        RECORD_MAGIC_NUMBER,
    )?;

    // write payload size
    let payload_size = record.payload_size as u32;
    filesystem::write(&mut file, &payload_size.to_le_bytes())?;

    // write payload data
    filesystem::write(&mut file, &record.data)?;

    // update row count
    let row_count = util::read_row_count(&mut file, table_index)?;
    let offset = TABLE_BLOCK_START_OFFSET +
        (TABLE_BLOCK_SIZE * table_index) as u64 +
        TABLE_MAGIC_NUMBER_SIZE as u64 +
        TABLE_NAME_SIZE as u64 +
        COLUMN_COUNT_SIZE as u64;
    filesystem::write_at(&mut file, offset, &(row_count + 1u32).to_le_bytes())?;


    Ok(())
}

pub fn read_record(
    id: u32,
    payload_size: usize,
    table_name: &str,
    db_name: &str,
) -> Result<Vec<u8>, Error> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // get table index
    let table_index = util::get_table_index(&mut file, table_name)?;

    // calculate offset
    let record_size = RECORD_MAGIC_NUMBER_SIZE +
        size_of::<u32>() +
        payload_size;
    let offset = RECORD_BLOCK_START_OFFSET +
        (TABLE_BLOCK_SIZE * table_index) as u64 +
        (record_size * (id - 1) as usize) as u64;

    // read record magic number
    let mut buffer = [0u8; RECORD_MAGIC_NUMBER_SIZE];
    filesystem::read_at(
        &mut file,
        offset,
        &mut buffer
    )?;

    if &buffer != RECORD_MAGIC_NUMBER {
        return Err(Error::Record(RecordError::RecordNotFound))
    }

    filesystem::move_current_file_cursor(
        &mut file,
        size_of::<u32>() as i64
    )?;

    let mut buffer = vec![0u8; payload_size];
    filesystem::read(
        &mut file,
        &mut buffer
    )?;

    Ok(buffer)
}

pub fn update_record(
    updated_data: Vec<u8>,
    id: u32,
    table_name: &str,
    db_name: &str,
) -> Result<(), Error> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;
    
    // get table index
    let table_index = util::get_table_index(&mut file, table_name)?;
    
    // calculate offset
    let record_size = 
        RECORD_MAGIC_NUMBER_SIZE + 
            size_of::<u32>() + 
            updated_data.len();
    let offset = RECORD_BLOCK_START_OFFSET +
        (TABLE_BLOCK_SIZE * table_index) as u64 +
        (record_size * (id - 1) as usize) as u64;
    
    // read record magic number
    let mut buffer = [0u8; RECORD_MAGIC_NUMBER_SIZE];
    filesystem::read_at(
        &mut file,
        offset,
        &mut buffer
    )?;
    
    // read payload size
    let mut buffer = [0u8; size_of::<u32>()];
    filesystem::read(&mut file, &mut buffer)?;
    
    // overwrite payload data
    filesystem::write(&mut file, &updated_data)?;
    
    Ok(())
}

pub fn delete_record(
    id: &u32,
    payload_size: usize,
    table_name: &str,
    db_name: &str,
) -> Result<(), Error> {
    // open .peb file
    let path = filesystem::create_file_path(db_name, FILE_EXTENSION);
    let mut file = filesystem::open_file(&path)?;

    // get table index
    let table_index = util::get_table_index(&mut file, table_name)?;

    // calculate offset
    let record_size = RECORD_MAGIC_NUMBER_SIZE +
        size_of::<u32>() +
        payload_size;
    let offset = RECORD_BLOCK_START_OFFSET +
        (TABLE_BLOCK_SIZE * table_index) as u64 +
        (record_size * (id - 1) as usize) as u64;

    // update magic number
    filesystem::write_at(
        &mut file,
        offset,
        RECORD_EMPTY_MAGIC_NUMBER,
    )?;

    // reset payload size
    filesystem::write(
        &mut file,
        &0u32.to_le_bytes()
    )?;

    // reset payload data
    filesystem::write(
        &mut file,
        &vec![0u8; payload_size]
    )?;

    Ok(())
}