#[derive(Debug)]
pub enum Command {
    Database(DatabaseCommand),
    Table(TableCommand),
    Record(RecordCommand),
    System(SystemCommand),
}

#[derive(Debug)]
pub enum DatabaseCommand {
    List,
    Create(String),
    Drop(String),
    Use(String),
}

#[derive(Debug)]
pub enum TableCommand {
    List,
    Create(String),
    Drop(String),
    Describe(String),
}

#[derive(Debug)]
pub enum RecordCommand {
    Select,
    Insert,
    Update,
    Delete,
}

#[derive(Debug)]
pub enum SystemCommand {
    Help,
    Exit,
}