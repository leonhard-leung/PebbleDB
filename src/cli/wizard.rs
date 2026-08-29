//! # CLI Wizard
//! Provides interactive prompts for collecting information required by CLI operations.

use crate::cli;
use crate::shared::error::Error;

/// # create_table_wizard
/// Collects table and column information.
///
/// The returned vector contains the table name followed by pairs of column names and normalized
/// data types.
pub fn create_table_wizard(table_name: &str) -> Result<Vec<String>, Error> {
    let mut data: Vec<String> = Vec::new();
    data.push(table_name.to_string());

    loop {
        // column name
        let column_name = cli::shell::read_input("Column Name: ");

        if column_name.is_empty() {
            cli::shell::print_warning("Column name cannot be empty.".to_string());
            continue;
        } else if data.iter().any(|col| col.eq_ignore_ascii_case(&column_name)) {
            cli::shell::print_warning(format!("Column name already exists: {}", column_name.to_lowercase()));
            continue;
        }
        data.push(column_name);

        // column data type
        let mut data_type = String::new();
        loop {
            let input = cli::shell::read_input("Column Type: ").to_lowercase();
            
            if cli::syntax::DATA_TYPES.contains(&input.as_str()) {
                data_type =  input;
                break;
            }
            
            cli::shell::print_warning(format!("Invalid data type: {}", input));
        }
        data.push(cli::syntax::normalize_data_type(&data_type).unwrap().to_string());
        
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
    Ok(data)
}