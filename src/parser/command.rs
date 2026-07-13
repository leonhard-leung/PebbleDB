use std::fmt;

#[derive(Debug)]
pub enum Action {
    Create(Target, String),
    Exit,
}

#[derive(Debug)]
pub enum Target {
    Database,
    Table,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Database => write!(f, "database"),
            Target::Table => write!(f, "table"),
        }
    }
}