use crate::shared::error::Error;
use tokio::net::TcpListener;
use crate::server::routes;

pub async fn start(
    shutdown: tokio::sync::oneshot::Receiver<()>
) -> Result<(), Error> {
    let app = routes::router();

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .map_err(Error::Io)?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = shutdown.await;
        })
        .await
        .map_err(Error::Io)?;

    Ok(())
}