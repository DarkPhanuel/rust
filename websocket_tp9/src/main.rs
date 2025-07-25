mod message;
mod server;
mod client;

pub use message::*;
pub use server::*;
pub use client::*;

use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "websocket_chat")]
#[command(about = "Un chat WebSocket en Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server {
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: SocketAddr,
    },
    Client {
        #[arg(short, long, default_value = "ws://127.0.0.1:8080")]
        url: String,
        #[arg(short, long)]
        username: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Server { addr } => {
            let server = WebSocketServer::new(addr);
            server.start().await?;
        }
        Commands::Client { url, username } => {
            let client = WebSocketClient::new(url, username);
            if let Err(e) = client.connect().await {
                eprintln!("Erreur client : {}", e);
            }
        }
    }

    Ok(())
}
