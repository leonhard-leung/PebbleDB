//! # Database Models
//! Defines the structures and data types used to represent database tables
//! and their columns.

use std::fmt;

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