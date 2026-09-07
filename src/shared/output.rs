pub enum Output {
    DatabaseList(Vec<String>),
    TablesList(Vec<String>),
    TableMetadata(Vec<String>),
    Record(Vec<(String, String)>),
    Message(String),
}