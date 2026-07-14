use std::fmt;

#[derive(Debug)]
pub enum Target {
    Database,
    Table,
}

#[derive(Debug)]
pub enum TargetForm {
    Singular,
    Plural,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Database => write!(f, "database"),
            Target::Table => write!(f, "table"),
        }
    }
}