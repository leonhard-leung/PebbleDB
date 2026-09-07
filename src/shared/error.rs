use std::fmt;
use crate::parser::grammar::Target;
use crate::parser::grammar::TargetForm;

// =================================================================================================
// Enums
// =================================================================================================
#[derive(Debug)]
pub enum Error {
    System(SystemError),
    Database(DatabaseError),
    Table(TableError),
    Record(RecordError),
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
    ParseError(ParseError),
    MissingTarget(TargetForm),
    MissingTargetName(Target),
    UnknownCommand(String),
    TooManyArguments(String),
    NoSelectedDatabase,
    NoInput,
}

#[derive(Debug)]
pub enum SystemError {

}

#[derive(Debug)]
pub enum DatabaseError {
    NoDatabaseSelected,
    DatabaseNotFound,
    DatabaseAlreadyExists,
}

#[derive(Debug)]
pub enum TableError {
    TableNotFound,
    TableAlreadyExists,
}

#[derive(Debug)]
pub enum RecordError {
    InvalidRecord(String),
    RecordLengthMismatch,
    RecordNotFound,
}

#[derive(Debug)]
pub enum ParseError {
    Int(std::num::ParseIntError),
    Float(std::num::ParseFloatError),
    Bool(std::str::ParseBoolError),
}

// =================================================================================================
// Display
// =================================================================================================
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingTarget(target_form) => {
                match target_form {
                    TargetForm::Singular => write!(f, "missing target (database/table)"),
                    TargetForm::Plural => write!(f, "missing target (databases/tables)"),
                }
            },
            Error::MissingTargetName(target) => write!(f, "missing {} name", target),
            Error::UnknownCommand(cmd) => write!(f, "unknown command '{}'", cmd),
            Error::TooManyArguments(cmd) => write!(f, "too many arguments: {}", cmd),
            Error::NoSelectedDatabase => write!(f, "no selected database"),
            Error::NoInput => write!(f, "missing input"),
            Error::System(err) => write!(f, "{}", err),
            Error::Database(err) => write!(f, "{}", err),
            Error::Table(err) => write!(f, "{}", err),
            Error::Record(err) => write!(f, "{}", err),
            
            Error::Io(err) => write!(f, "{}", err),
            Error::Utf8(err) => write!(f, "{}", err),
            Error::ParseError(err) => write!(f, "{}", err),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "system error")
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::NoDatabaseSelected => write!(f, "No database selected, try \"use <name>\" first"),
            DatabaseError::DatabaseNotFound => write!(f, "Database not found"),
            DatabaseError::DatabaseAlreadyExists => write!(f, "Database already exists"),
        }
    }
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableError::TableNotFound => write!(f, "Table not found"),
            TableError::TableAlreadyExists => write!(f, "Table already exists"),
        }
    }
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordError::InvalidRecord(err) => write!(f, "Invalid record\n{}", err),
            RecordError::RecordLengthMismatch => write!(f, " record values does not match column count"),
            RecordError::RecordNotFound => write!(f, "Record not found"),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Int(err) => write!(f, "Invalid integer value\n{}", err),
            ParseError::Float(err) => write!(f, "Invalid float value\n{}", err),
            ParseError::Bool(err) => write!(f, "Invalid boolean value\n{}", err),
        }
    }
}

// =================================================================================================
// Error Conversion
// =================================================================================================
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error { Error::Utf8(err) }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error { Error::ParseError(ParseError::Int(err)) }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Error { Error::ParseError(ParseError::Float(err)) }
}

impl From<std::str::ParseBoolError> for Error {
    fn from(err: std::str::ParseBoolError) -> Error { Error::ParseError(ParseError::Bool(err)) }
}