use crate::cli;
use crate::database::model::{Column, DataType, Table};
use crate::shared::error::Error;

pub fn create_table_wizard(table_name: &str) -> Result<Table, Error> {
    // column details
    let mut columns: Vec<Column> = Vec::new();
    loop {
        // column name
        let column_name = cli::shell::read_input("Column Name: ");

        if column_name.is_empty() {
            cli::shell::print_warning("Column name cannot be empty.".to_string());
            continue;
        } else if columns.iter().any(|col| col.name.eq_ignore_ascii_case(&column_name)) {
            cli::shell::print_warning(format!("Column name already exists: {}", column_name.to_lowercase()));
            continue;
        }
        
        // TODO: CHANGE THIS, THE WIZARD SHOULD NOT BE ABLE TO CREATE DATATYPE AND COLUMN STRUCTS, JUST PURE STRINGS AND NUMBERS ARE ONLY ALLOWED

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