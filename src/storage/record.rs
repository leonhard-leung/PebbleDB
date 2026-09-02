//! # Record
//! Provides operations for selecting, inserting, deleting, and updating records.

use crate::database::model::Record;
use crate::shared::error::Error;

pub fn insert_record(
    record: Record,
    table_name: &str,
    db_name: &str,
) -> Result<(), Error> {
    // TODO: implement this function (Review api.rs and model.rs)
    
    Ok(())
}