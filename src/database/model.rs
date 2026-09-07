//! # Database Models
//! Defines the structures and data types used to represent database tables
//! and their columns.

use std::fmt;
use crate::shared::error::Error;
// =================================================================================================
// Table Model
// =================================================================================================

/// Represents a database table with its columns and row count.
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub row_count: u32,
}

/// Represents a column within a database table.
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

/// Defines the supported data types for database columns.
pub enum DataType {
    Integer,
    Float,
    Boolean,
    Text,
}

const INTEGER: u8 = 0;
const FLOAT: u8 = 1;
const BOOLEAN: u8 = 2;
const TEXT: u8 = 3;

impl DataType {
    /// Returns the unique identifier for the data type.
    pub fn id(&self) -> u8 {
        match self {
            DataType::Integer => INTEGER,
            DataType::Float => FLOAT,
            DataType::Boolean => BOOLEAN,
            DataType::Text => TEXT,
        }
    }

    /// Returns the maximum length of the data type in bytes.
    pub fn size(&self) -> usize {
        match self {
            DataType::Integer => 4,
            DataType::Float => 4,
            DataType::Boolean => 1,
            DataType::Text => 255,
        }
    }

    /// Validates the input string against the data type's constraints.
    /// Returns a tuple containing a boolean indicating success and an error message.
    pub fn validate(&self, input: &str) -> (bool, String) {
        match self {
            DataType::Integer => {
                if input.parse::<i32>().is_ok() {
                    (true, String::new())
                } else {
                    (false, "Invalid integer value".to_string())
                }
            },
            DataType::Float => {
                if input.parse::<f32>().is_ok() {
                    (true, String::new())
                } else {
                    (false, "Invalid float value".to_string())
                }
            },
            DataType::Boolean => {
                if input.parse::<bool>().is_ok() {
                    (true, String::new())
                } else {
                    (false, "Invalid boolean value".to_string())
                }
            },
            DataType::Text => {
                if input.len() <= self.size() {
                    (true, String::new())
                } else {
                    (false, "Text value exceeds maximum length".to_string())
                }
            },
        }
    }

    /// Converts a data type identifier to its corresponding DataType enum value.
    pub fn from_id(id: u8) -> DataType {
        match id {
            INTEGER => DataType::Integer,
            FLOAT => DataType::Float,
            BOOLEAN => DataType::Boolean,
            TEXT => DataType::Text,
            _ => panic!("Invalid data type id"),
        }
    }

    /// Converts a normalized data type string to its corresponding DataType enum value.
    pub fn from_str(s: &str) -> DataType {
        match s {
            "Integer" => DataType::Integer,
            "Float" => DataType::Float,
            "Boolean" => DataType::Boolean,
            "Text" => DataType::Text,
            _ => panic!("Invalid data type string"),
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer => write!(f, "Integer"),
            DataType::Float => write!(f, "Float"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::Text => write!(f, "Text"),
        }
    }
}

// =================================================================================================
// Record Model
// =================================================================================================

/// Represents a single record in a table with its data and columns.
pub struct Record {
    pub data: Vec<String>,
    pub columns: Vec<Column>,
}

pub struct SerializedRecord {
    pub data: Vec<u8>,
    pub payload_size: usize
}

impl Record {
    pub fn serialize(
        &self
    ) -> Result<SerializedRecord, Error> {
        let mut serialized = Vec::new();

        for (entry, column) in self.data.iter().zip(self.columns.iter()) {
            
            match column.data_type {
                DataType::Integer => {
                    let value = entry.parse::<i32>()?;
                    serialized.extend_from_slice(&value.to_le_bytes());
                },
                DataType::Float => {
                    let value = entry.parse::<f32>()?;
                    serialized.extend_from_slice(&value.to_le_bytes());
                },
                DataType::Boolean => {
                    let value = entry.parse::<bool>()?;
                    serialized.push(value as u8);
                },
                DataType::Text => {
                    let mut buffer = vec![0u8; DataType::Text.size()];
                    let bytes = entry.as_bytes();

                    buffer[..bytes.len()].copy_from_slice(bytes);

                    serialized.extend_from_slice(&buffer);
                },
            }
        }

        let payload_size = serialized.len();

        Ok(SerializedRecord {
            data: serialized,
            payload_size
        })
    }
}

impl SerializedRecord {
    pub fn deserialize(
        &self,
        columns: &[(String, u8)]
    ) -> Result<Record, Error> {
        let mut deserialized = Vec::new();
        let mut offset = 0;
        let mut cols: Vec<Column> = Vec::new();

        for (column_name, data_type_id) in columns.iter() {
            let data_type = DataType::from_id(*data_type_id);
            
            cols.push(Column{
                name: column_name.to_owned(),
                data_type: DataType::from_id(*data_type_id)
            });

            match data_type {
                DataType::Integer => {
                    let bytes = &self.data[offset..offset + DataType::Integer.size()];
                    let value = i32::from_le_bytes(bytes.try_into().unwrap());

                    deserialized.push(value.to_string());
                    offset += DataType::Integer.size();
                },
                DataType::Float => {
                    let bytes = &self.data[offset..offset + DataType::Float.size()];
                    let value = f32::from_le_bytes(bytes.try_into().unwrap());

                    deserialized.push(value.to_string());
                    offset += DataType::Float.size();
                },
                DataType::Boolean => {
                    let value = self.data[offset] != 0;

                    deserialized.push(value.to_string());
                    offset += DataType::Boolean.size();
                },
                DataType::Text => {
                    let bytes = &self.data[offset..offset + DataType::Text.size()];
                    let value = std::str::from_utf8(&bytes)?.trim_end_matches('\0');
                    
                    deserialized.push(value.to_string());
                    offset += DataType::Text.size();
                },
            }
        }
        Ok(Record { 
            data: deserialized,
            columns: cols
        })
    }
}