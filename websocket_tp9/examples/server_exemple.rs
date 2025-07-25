use websocket_tp9::WebSocketServer;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let server = WebSocketServer::new(addr);

    println!("Démarrage du serveur WebSocket sur {}", addr);
    server.start().await?;

    Ok(())
}
