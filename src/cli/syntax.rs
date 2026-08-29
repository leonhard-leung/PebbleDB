//! # CLI Syntax
//! Defines keywords and aliases recognized by the CLI.

/// Supported data type names and aliases accepted by the CLI.
pub const DATA_TYPES: &[&str] = &[
    "int",
    "integer",
    "float",
    "double",
    "boolean",
    "bool",
    "text"
];

/// # normalize_data_type
/// Converts a CLI data type name or alias into its normalized name.
pub fn normalize_data_type(
    input: &str
) -> Option<&'static str> {
    match input {
        "int" | "integer" => Some("Integer"),
        "float" | "double" => Some("Float"),
        "boolean" | "bool" => Some("Boolean"),
        "text" => Some("Text"),
        _ => None
    }
}