use websocket_tp9::WebSocketClient;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <username>", args[0]);
        std::process::exit(1);
    }

    let username = args[1].clone();
    let client = WebSocketClient::new("ws://127.0.0.1:8080".to_string(), username);

    if let Err(e) = client.connect().await {
        eprintln!("Erreur client : {}", e);
    }

    Ok(())
}
