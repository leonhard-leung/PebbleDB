use crate::cli;
use crate::database::model::{Column, DataType, Table};
use crate::shared::error::Error;

pub fn create_table_wizard(table_name: &str) -> Result<Table, Error> {
    // column details
    let mut columns: Vec<Column> = Vec::new();
    loop {
        // column name
        let column_name = cli::shell::read_input("Column Name: ");

        // column data type
        let data_type: DataType;
        loop {
            let input = cli::shell::read_input("Column Type: ");
            match input.to_lowercase().as_str() {
                "int" => data_type = DataType::Integer,
                "float" => data_type = DataType::Float,
                "boolean" => data_type = DataType::Boolean,
                "text" => data_type = DataType::Text,
                _ => continue,
            }
            break
        }
        
        // push column to vector
        let column = Column {
            name: column_name,
            data_type,
        };
        columns.push(column);
        
        // add another column
        let mut input: String;
        loop {
            input = cli::shell::read_input("Add Another Column? <y/n>: ").to_lowercase();
            
            if input == "y" || input == "n" { break; }
        }
        
        if input == "y" {
            continue;
        }
        break
    }
    
    let table = Table {
        name: table_name.to_string(),
        columns,
        row_count: 0
    };
    
    Ok(table)
}