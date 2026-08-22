use std::fmt;
use crate::parser::grammar::Target;
use crate::parser::grammar::TargetForm;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
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
}

#[derive(Debug)]
pub enum TableError {

}

#[derive(Debug)]
pub enum RecordError {

}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error { Error::Utf8(err) }
}


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
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::NoDatabaseSelected => write!(f, "No database selected, try \"use <name>\" first"),
        }
    }
}