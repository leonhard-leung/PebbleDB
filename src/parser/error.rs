use std::fmt;
use crate::parser::command::Target;

#[derive(Debug)]
pub enum Error {
    MissingTarget,
    MissingTargetName(Target),
    UnknownCommand(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingTarget => write!(f, "missing target (database/table)"),
            Error::MissingTargetName(target) => write!(f, "missing {} name", target),
            Error::UnknownCommand(cmd) => write!(f, "unknown command '{}'", cmd),
        }
    }
}