use simple_redis::stream_handler;
use simple_redis::Backend;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    info!("Simple-Redis-Server is listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    let backend = Backend::new();
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from:{}", raddr);
        let cloned_backend = backend.clone();
        tokio::spawn(async move {
            match stream_handler(stream, cloned_backend).await {
                Ok(_) => {
                    info!("Connection from {} exited", raddr);
                }
                Err(e) => {
                    warn!("handle error for {}:{:?}", raddr, e);
                }
            }
        });
    }
}
