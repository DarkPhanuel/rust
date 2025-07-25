use restychat_tp8::server::ChatServer;
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration des logs
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Adresse d'écoute par défaut
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    println!("🚀 Démarrage du serveur RustChat...");
    println!("📡 Écoute sur: {}", addr);
    println!("🔧 Appuyez sur Ctrl+C pour arrêter");
    println!();

    let server = ChatServer::new();
    server.start(&addr).await?;

    Ok(())
}
