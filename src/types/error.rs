use std::fmt;
use crate::parser::grammar::Target;
use crate::parser::grammar::TargetForm;

#[derive(Debug)]
pub enum Error {
    MissingTarget(TargetForm),
    MissingTargetName(Target),
    UnknownCommand(String),
    TooManyArguments(String),
    NoInput,
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
            Error::NoInput => write!(f, "missing input"),
        }
    }
}