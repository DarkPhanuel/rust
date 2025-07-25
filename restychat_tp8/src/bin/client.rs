use restychat_tp8::client::ChatClient;
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration des logs (niveau WARN pour moins de verbosité côté client)
    tracing_subscriber::fmt()
        .with_max_level(Level::WARN)
        .init();

    // Adresse du serveur par défaut
    let server_addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    println!("🌟 Client RustChat");
    println!("🔗 Connexion à: {}", server_addr);
    println!();

    let mut client = ChatClient::new();

    if let Err(e) = client.connect(&server_addr).await {
        eprintln!("❌ Erreur de connexion: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
