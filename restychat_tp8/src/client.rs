use crate::{ClientMessage, ClientState, ProtocolMessage, ServerMessage};
use anyhow::Result;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tracing::{error, info};

pub struct ChatClient {
}

impl ChatClient {
    pub fn new() -> Self {
        Self {
        }
    }

    async fn handle_server_message(msg: &ProtocolMessage, client_state: &mut ClientState, tx: &mpsc::Sender<ClientMessage>) -> Result<bool> {
        let server_msg = ServerMessage::from_protocol_message(msg)?;

        match server_msg {
            ServerMessage::ConnectResponse { success, message } => {
                if success {
                    println!("✅ {}", message);
                    println!("📋 Commandes disponibles:");
                    println!("  /msg <utilisateur> <message> - Envoyer un message privé");
                    println!("  /users - Lister les utilisateurs connectés");
                    println!("  /quit - Quitter");
                    println!("  Tapez simplement votre message pour l'envoyer à tous");
                    println!();
                    return Ok(true);
                } else {
                    println!("❌ {}", message);
                    *client_state = ClientState::Disconnected;
                    // Redemander le nom d'utilisateur
                    Self::ask_username(tx, client_state).await;
                    return Ok(false);
                }
            },
            ServerMessage::MessageBroadcast { from, content } => {
                println!("💬 {}: {}", from, content);
            },
            ServerMessage::PrivateMessageDelivery { from, content } => {
                println!("📨 [Privé] {}: {}", from, content);
            },
            ServerMessage::UserList { users } => {
                println!("👥 Utilisateurs connectés ({}):", users.len());
                for user in users {
                    println!("  - {}", user);
                }
            },
            ServerMessage::UserJoined { username } => {
                println!("👋 {} a rejoint le chat", username);
            },
            ServerMessage::UserLeft { username } => {
                println!("👋 {} a quitté le chat", username);
            },
            ServerMessage::Error { message } => {
                println!("❌ Erreur: {}", message);
            },
        }
        Ok(true)
    }

    async fn ask_username(tx: &mpsc::Sender<ClientMessage>, client_state: &mut ClientState) {
        use tokio::io::{stdin, AsyncBufReadExt, BufReader};
        let stdin = stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();
        loop {
            println!("👤 Entrez votre nom d'utilisateur:");
            line.clear();
            if reader.read_line(&mut line).await.is_err() {
                return;
            }
            let username = line.trim().to_string();
            if username.is_empty() {
                println!("❌ Le nom d'utilisateur ne peut pas être vide");
                continue;
            }
            let connect_msg = ClientMessage::Connect { username: username.clone() };
            if tx.send(connect_msg).await.is_err() {
                return;
            }
            *client_state = ClientState::Connected(username);
            break;
        }
    }

    pub async fn connect(&mut self, server_addr: &str) -> Result<()> {
        let stream = TcpStream::connect(server_addr).await?;
        info!("🔗 Connecté au serveur {}", server_addr);

        let (mut reader, mut writer) = stream.into_split();

        // Canal pour envoyer des messages au serveur
        let (tx, mut rx) = mpsc::channel::<ClientMessage>(100);
        let tx_clone = tx.clone();
        // On ne partage plus client_state entre tâches, il reste local à l'input utilisateur

        // Task pour lire les messages du serveur
        let receive_task = tokio::spawn(async move {
            loop {
                match ProtocolMessage::read_from(&mut reader).await {
                    Ok(msg) => {
                        // On ne gère plus client_state ici
                        let _ = Self::handle_server_message(&msg, &mut ClientState::Disconnected, &tx_clone).await;
                    },
                    Err(e) => {
                        error!("❌ Erreur lecture message: {}", e);
                        break;
                    }
                }
            }
        });

        // Task pour envoyer des messages au serveur
        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Ok(protocol_msg) = msg.to_protocol_message() {
                    if protocol_msg.write_to(&mut writer).await.is_err() {
                        break;
                    }
                }
            }
        });

        // Task pour lire l'input utilisateur
        let input_task = tokio::spawn(async move {
            let mut client_state = ClientState::Disconnected;
            Self::ask_username(&tx, &mut client_state).await;
            Self::run_user_interface(tx).await;
        });

        // Attendre qu'une des tâches se termine
        tokio::select! {
            _ = receive_task => {},
            _ = send_task => {},
            _ = input_task => {},
        }

        Ok(())
    }

    async fn run_user_interface(tx: mpsc::Sender<ClientMessage>) {
        let stdin = stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        // Boucle principale de chat
        loop {
            line.clear();
            if reader.read_line(&mut line).await.is_err() {
                break;
            }

            let input = line.trim();
            if input.is_empty() {
                continue;
            }

            let message = if input.starts_with('/') {
                Self::parse_command(input)
            } else {
                Some(ClientMessage::PublicMessage {
                    content: input.to_string(),
                })
            };

            if let Some(msg) = message {
                // Vérifier si c'est une commande de déconnexion
                if matches!(msg, ClientMessage::Disconnect) {
                    let _ = tx.send(msg).await;
                    break;
                }

                if tx.send(msg).await.is_err() {
                    break;
                }
            }
        }
    }

    fn parse_command(input: &str) -> Option<ClientMessage> {
        let parts: Vec<&str> = input.splitn(3, ' ').collect();

        match parts[0] {
            "/msg" | "/pm" => {
                if parts.len() >= 3 {
                    let to = parts[1].to_string();
                    let content = parts[2].to_string();
                    Some(ClientMessage::PrivateMessage { to, content })
                } else {
                    println!("❌ Usage: /msg <utilisateur> <message>");
                    None
                }
            },
            "/users" | "/list" => {
                Some(ClientMessage::ListUsers)
            },
            "/quit" | "/exit" => {
                println!("👋 Au revoir!");
                Some(ClientMessage::Disconnect)
            },
            "/help" => {
                println!("📋 Commandes disponibles:");
                println!("  /msg <utilisateur> <message> - Envoyer un message privé");
                println!("  /users - Lister les utilisateurs connectés");
                println!("  /quit - Quitter");
                println!("  /help - Afficher cette aide");
                println!("  Tapez simplement votre message pour l'envoyer à tous");
                None
            },
            _ => {
                println!("❌ Commande inconnue: {}. Tapez /help pour l'aide", parts[0]);
                None
            }
        }
    }
}
