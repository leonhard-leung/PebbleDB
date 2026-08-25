///////////////////////////////////////////////////////////////////////////////////////////////////
/// # TABLE MODEL
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # Table Structure
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub row_count: u32,
}

/// # Column Structure
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

/// # Data Type Enumerations
pub enum DataType {
    Integer,
    Float,
    Boolean,
    Text,
}

impl DataType {
    pub fn id(&self) -> u8 {
        match self {
            DataType::Integer => 0,
            DataType::Float => 1,
            DataType::Boolean => 2,
            DataType::Text => 3,
        }
    }
}


pub enum Output {
    Databases(Vec<String>),
    Tables(Vec<String>),
    TableMetadata(Vec<String>),
    Message(String),
}