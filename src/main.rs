use anyhow::Result;
use simple_redis::{backend::Backend, network};
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    info!("Simple-Redis-Server is listening on {}", addr);
    let backend = Backend::default();
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection form: {}", raddr);
        let cloned_backend = backend.clone();
        tokio::spawn(async move {
            match network::stream_handler(stream, cloned_backend).await {
                Ok(_) => {
                    info!("Connection form {} exited", raddr);
                }
                Err(e) => {
                    warn!("handle error for {}: {:?}", raddr, e);
                }
            }
        });
    }
}
