// =================================================================================================
// Table Model
// =================================================================================================

use std::fmt;

pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub row_count: u32,
}

pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

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
    pub fn id(&self) -> u8 {
        match self {
            DataType::Integer => INTEGER,
            DataType::Float => FLOAT,
            DataType::Boolean => BOOLEAN,
            DataType::Text => TEXT,
        }
    }
    
    pub fn from_id(id: u8) -> DataType {
        match id {
            INTEGER => DataType::Integer,
            FLOAT => DataType::Float,
            BOOLEAN => DataType::Boolean,
            TEXT => DataType::Text,
            _ => panic!("Invalid data type id"),
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