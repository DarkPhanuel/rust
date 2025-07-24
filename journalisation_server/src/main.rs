use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::{Local};
use std::sync::{Arc, Mutex};
use std::fs;
struct JournalisationServer {
    port: u16,
    fichier_de_log: Arc<Mutex<std::fs::File>>,
}

impl JournalisationServer {
    fn nouveau(port: u16) -> std::io::Result<Self> {
        fs::create_dir_all("logs")?;
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/server.log")?;

        Ok(JournalisationServer {
            port,
            fichier_de_log: Arc::new(Mutex::new(log_file)),
        })
    }

    async fn demarrer(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        println!("🚀 Serveur de journalisation démarré sur le port {}", self.port);
        println!("📝 Les logs seront enregistrés dans logs/server.log");
        self.log_message("SERVER", "Serveur de journalisation démarré").await;



       /*for i in 1..=5 {
            println!("🔄 Initialisation en cours... ({}/{})", i, 5);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }*/
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("🔗 Nouvelle connexion depuis: {}", addr);
                    let log_file_clone = Arc::clone(&self.fichier_de_log);
                    tokio::spawn(async move {
                        Self::handle_client(stream, addr.to_string(), log_file_clone).await;
                    });
                }
                Err(e) => {
                    println!("❌ Erreur lors de l'acceptation de connexion: {}", e);
                }
            }
        }
    }
    async fn handle_client(
        mut stream: TcpStream,
        client_addr: String,
        log_file: Arc<Mutex<std::fs::File>>
    ) {
        println!("👤 Traitement du client: {}", client_addr);
        let welcome_msg = "Bienvenue sur le serveur de journalisation!\nVos messages seront enregistrés avec horodatage.\nTapez 'quit' pour vous déconnecter.\n";
        if let Err(e) = stream.write_all(welcome_msg.as_bytes()).await {
            println!("❌ Erreur envoi message bienvenue à {}: {}", client_addr, e);
            return;
        }
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    println!("🔌 Client {} déconnecté", client_addr);
                    Self::write_to_log(&log_file, &client_addr, "Client déconnecté").await;
                    break;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    let message = message.trim();
                    println!("📨 Message de {}: {}", client_addr, message);
                    if message.to_lowercase() == "quit" {
                        println!("👋 Client {} a demandé la déconnexion", client_addr);
                        let goodbye_msg = "Au revoir!\n";
                        let _ = stream.write_all(goodbye_msg.as_bytes()).await;
                        Self::write_to_log(&log_file, &client_addr, "Client déconnecté (quit)").await;
                        break;
                    }
                    Self::write_to_log(&log_file, &client_addr, message).await;
                    let confirmation = format!("✅ Message reçu et enregistré: {}\n", message);
                    if let Err(e) = stream.write_all(confirmation.as_bytes()).await {
                        println!("❌ Erreur envoi confirmation à {}: {}", client_addr, e);
                        break;
                    }
                }
                Err(e) => {
                    println!("❌ Erreur lecture depuis {}: {}", client_addr, e);
                    break;
                }
            }
        }
    }

    async fn write_to_log(
        log_file: &Arc<Mutex<std::fs::File>>,
        client_addr: &str,
        message: &str
    ) {
        let timestamp = Local::now();
        let formatted_timestamp = timestamp.format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("[{}] [{}] {}\n", formatted_timestamp, client_addr, message);
        match log_file.lock() {
            Ok(mut file) => {
                if let Err(e) = file.write_all(log_entry.as_bytes()) {
                    println!("❌ Erreur écriture dans le fichier de log: {}", e);
                } else {
                    let _ = file.flush();
                    println!("📝 Log enregistré: [{}] [{}] {}", formatted_timestamp, client_addr, message);
                }
            }
            Err(e) => {
                println!("❌ Erreur accès au fichier de log: {}", e);
            }
        }
    }
    async fn log_message(&self, source: &str, message: &str) {
        Self::write_to_log(&self.fichier_de_log, source, message).await;
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("🌟 === SERVEUR DE JOURNALISATION ASYNCHRONE ===");
    println!("📋 Fonctionnalités:");
    println!("   • Écoute sur port TCP");
    println!("   • Gestion simultanée de multiple clients");
    println!("   • Enregistrement avec horodatage");
    println!("   • Fichier de log automatique");
    println!();
    let server = JournalisationServer::nouveau(8080)?;
    server.demarrer().await?;

    Ok(())
}

