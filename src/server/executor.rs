use crate::runtime::session::Session;
use crate::shared::command::ServerCommand;
use crate::shared::error::Error;
use crate::shared::output::Output;

/// # Execute
pub fn execute_server(
    command: ServerCommand,
    session: &mut Session
) -> Result<Output, Error> {
    match command {
        ServerCommand::Start => {
            let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

            tokio::spawn(async {
                if let Err(error) = crate::server::runtime::start(shutdown_rx).await {
                    eprintln!("Server error: {}", error);
                }
            });

            session.server.shutdown = Some(shutdown_tx);

            Ok(Output::Message("Server started at http://127.0.0.1:3000".to_string()))
        },
        ServerCommand::Stop => {
            if let Some(shutdown_tx) = session.server.shutdown.take() {
                let _ = shutdown_tx.send(());

                Ok(Output::Message("Server stopped".to_string()))
            } else {
                Ok(Output::Message("Server is not running".to_string()))
            }
        },
        ServerCommand::Status => {
            if session.server.shutdown.is_some() {
                Ok(Output::Message("Server is running".to_string()))
            } else {
                Ok(Output::Message("Server is not running".to_string()))
            }
        },
    }
}

