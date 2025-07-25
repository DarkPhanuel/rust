use crate::{
    ClientMessage, ProtocolMessage, ServerClientState, ServerMessage,
    MSG_CONNECT, MSG_DISCONNECT, MSG_LIST_USERS, MSG_PRIVATE_MESSAGE, MSG_PUBLIC_MESSAGE,
};
use anyhow::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, RwLock};
use tracing::{error, info, warn};

type ClientId = u32;

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub id: ClientId,
    pub addr: SocketAddr,
    pub state: ServerClientState,
    pub sender: broadcast::Sender<ServerMessage>,
}

pub struct ChatServer {
    clients: Arc<RwLock<HashMap<ClientId, ClientInfo>>>,
    next_client_id: Arc<RwLock<ClientId>>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
}

impl ChatServer {
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(1000);

        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            next_client_id: Arc::new(RwLock::new(1)),
            broadcast_tx,
        }
    }

    pub async fn start(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        info!("🚀 Serveur RustChat démarré sur {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("📱 Nouvelle connexion de {}", addr);
                    let server = self.clone();
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_client(stream, addr).await {
                            error!("❌ Erreur avec client {}: {}", addr, e);
                        }
                    });
                },
                Err(e) => {
                    error!("❌ Erreur d'acceptation de connexion: {}", e);
                }
            }
        }
    }

    async fn handle_client(&self, stream: TcpStream, addr: SocketAddr) -> Result<()> {
        let client_id = {
            let mut next_id = self.next_client_id.write().await;
            let id = *next_id;
            *next_id += 1;
            id
        };

        let (client_tx, mut client_rx) = broadcast::channel(100);

        let client_info = ClientInfo {
            id: client_id,
            addr,
            state: ServerClientState::WaitingAuth,
            sender: client_tx,
        };

        {
            let mut clients = self.clients.write().await;
            clients.insert(client_id, client_info);
        }

        let (mut reader, mut writer) = stream.into_split();

        // Task pour envoyer les messages au client
        let send_task = tokio::spawn(async move {
            while let Ok(msg) = client_rx.recv().await {
                if let Ok(protocol_msg) = msg.to_protocol_message() {
                    if protocol_msg.write_to(&mut writer).await.is_err() {
                        break;
                    }
                }
            }
        });

        // Task pour recevoir les messages du client
        let server_clone = self.clone();
        let receive_task = tokio::spawn(async move {
            loop {
                match ProtocolMessage::read_from(&mut reader).await {
                    Ok(msg) => {
                        if server_clone.process_client_message(client_id, &msg).await.is_err() {
                            break;
                        }

                        // Déconnexion explicite
                        if msg.msg_type == MSG_DISCONNECT {
                            break;
                        }
                    },
                    Err(_) => break,
                }
            }
        });

        // Attendre que l'une des tâches se termine
        tokio::select! {
            _ = send_task => {},
            _ = receive_task => {},
        }

        // Nettoyage
        self.remove_client(client_id).await;
        info!("📤 Client {} déconnecté", addr);

        Ok(())
    }

    async fn process_client_message(&self, client_id: ClientId, msg: &ProtocolMessage) -> Result<()> {
        match msg.msg_type {
            MSG_CONNECT => {
                let client_msg = ClientMessage::from_protocol_message(msg)?;
                if let ClientMessage::Connect { username } = client_msg {
                    self.handle_connect(client_id, username).await?;
                }
            },
            MSG_PUBLIC_MESSAGE => {
                let client_msg = ClientMessage::from_protocol_message(msg)?;
                if let ClientMessage::PublicMessage { content } = client_msg {
                    self.handle_public_message(client_id, content).await?;
                }
            },
            MSG_PRIVATE_MESSAGE => {
                let client_msg = ClientMessage::from_protocol_message(msg)?;
                if let ClientMessage::PrivateMessage { to, content } = client_msg {
                    self.handle_private_message(client_id, to, content).await?;
                }
            },
            MSG_LIST_USERS => {
                self.handle_list_users(client_id).await?;
            },
            MSG_DISCONNECT => {
                info!("👋 Déconnexion explicite du client {}", client_id);
            },
            _ => {
                warn!("⚠️ Type de message inconnu: {}", msg.msg_type);
            }
        }
        Ok(())
    }

    async fn handle_connect(&self, client_id: ClientId, username: String) -> Result<()> {
        // Vérifier si le nom d'utilisateur est déjà pris
        let is_taken = {
            let clients = self.clients.read().await;
            clients.values().any(|client| {
                matches!(client.state, ServerClientState::Authenticated(ref name) if name == &username)
            })
        };

        let response = if is_taken {
            ServerMessage::ConnectResponse {
                success: false,
                message: format!("Nom d'utilisateur '{}' déjà pris", username),
            }
        } else {
            // Mettre à jour l'état du client
            {
                let mut clients = self.clients.write().await;
                if let Some(client) = clients.get_mut(&client_id) {
                    client.state = ServerClientState::Authenticated(username.clone());
                }
            }

            // Notifier les autres clients
            let notification = ServerMessage::UserJoined {
                username: username.clone(),
            };
            self.broadcast_to_authenticated(notification, Some(client_id)).await;

            info!("✅ Utilisateur '{}' connecté", username);

            ServerMessage::ConnectResponse {
                success: true,
                message: format!("Bienvenue, {}!", username),
            }
        };

        self.send_to_client(client_id, response).await;
        Ok(())
    }

    async fn handle_public_message(&self, client_id: ClientId, content: String) -> Result<()> {
        let username = {
            let clients = self.clients.read().await;
            if let Some(client) = clients.get(&client_id) {
                if let ServerClientState::Authenticated(ref username) = client.state {
                    username.clone()
                } else {
                    self.send_error(client_id, "Vous devez être connecté pour envoyer des messages").await;
                    return Ok(());
                }
            } else {
                return Ok(());
            }
        };

        let message = ServerMessage::MessageBroadcast {
            from: username.clone(),
            content,
        };

        info!("💬 Message public de {}: {:?}", username, message);
        self.broadcast_to_authenticated(message, None).await;
        Ok(())
    }

    async fn handle_private_message(&self, client_id: ClientId, to: String, content: String) -> Result<()> {
        let from_username = {
            let clients = self.clients.read().await;
            if let Some(client) = clients.get(&client_id) {
                if let ServerClientState::Authenticated(ref username) = client.state {
                    username.clone()
                } else {
                    self.send_error(client_id, "Vous devez être connecté pour envoyer des messages").await;
                    return Ok(());
                }
            } else {
                return Ok(());
            }
        };

        // Trouver le destinataire
        let target_client_id = {
            let clients = self.clients.read().await;
            clients.iter()
                .find(|(_, client)| {
                    matches!(client.state, ServerClientState::Authenticated(ref username) if username == &to)
                })
                .map(|(id, _)| *id)
        };

        if let Some(target_id) = target_client_id {
            let message = ServerMessage::PrivateMessageDelivery {
                from: from_username.clone(),
                content,
            };

            info!("📨 Message privé de {} vers {}", from_username, to);
            self.send_to_client(target_id, message).await;
        } else {
            self.send_error(client_id, &format!("Utilisateur '{}' introuvable", to)).await;
        }

        Ok(())
    }

    async fn handle_list_users(&self, client_id: ClientId) -> Result<()> {
        let users = {
            let clients = self.clients.read().await;
            clients.values()
                .filter_map(|client| {
                    if let ServerClientState::Authenticated(ref username) = client.state {
                        Some(username.clone())
                    } else {
                        None
                    }
                })
                .collect()
        };

        let response = ServerMessage::UserList { users };
        self.send_to_client(client_id, response).await;
        Ok(())
    }

    async fn send_to_client(&self, client_id: ClientId, message: ServerMessage) {
        let clients = self.clients.read().await;
        if let Some(client) = clients.get(&client_id) {
            let _ = client.sender.send(message);
        }
    }

    async fn send_error(&self, client_id: ClientId, error_message: &str) {
        let error = ServerMessage::Error {
            message: error_message.to_string(),
        };
        self.send_to_client(client_id, error).await;
    }

    async fn broadcast_to_authenticated(&self, message: ServerMessage, exclude_client: Option<ClientId>) {
        let clients = self.clients.read().await;
        for (id, client) in clients.iter() {
            if let Some(exclude_id) = exclude_client {
                if *id == exclude_id {
                    continue;
                }
            }

            if matches!(client.state, ServerClientState::Authenticated(_)) {
                let _ = client.sender.send(message.clone());
            }
        }
    }

    async fn remove_client(&self, client_id: ClientId) {
        let username = {
            let mut clients = self.clients.write().await;
            if let Some(client) = clients.remove(&client_id) {
                if let ServerClientState::Authenticated(username) = client.state {
                    Some(username)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(username) = username {
            let notification = ServerMessage::UserLeft { username };
            self.broadcast_to_authenticated(notification, None).await;
        }
    }
}

impl Clone for ChatServer {
    fn clone(&self) -> Self {
        Self {
            clients: self.clients.clone(),
            next_client_id: self.next_client_id.clone(),
            broadcast_tx: self.broadcast_tx.clone(),
        }
    }
}
