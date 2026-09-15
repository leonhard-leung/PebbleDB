pub struct Session {
    pub current_database: Option<String>,
    pub server: ServerState
}

pub struct ServerState {
    pub running: bool,
    pub shutdown: Option<tokio::sync::oneshot::Sender<()>>,
}