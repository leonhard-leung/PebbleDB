pub enum Output {
    DatabaseList(Vec<String>),
    TablesList(Vec<String>),
    TableMetadata(Vec<String>),
    Message(String),
}