use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, RwLock, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage};
use futures_util::{SinkExt, StreamExt};
use tracing::{info, warn, error, debug};
use crate::message::{Message, ClientInfo};

type Clients = Arc<RwLock<HashMap<String, ClientInfo>>>;
type Broadcaster = broadcast::Sender<(String, Message)>;

pub struct WebSocketServer {
    addr: SocketAddr,
    clients: Clients,
    broadcaster: Broadcaster,
}

impl WebSocketServer {
    pub fn new(addr: SocketAddr) -> Self {
        let clients = Arc::new(RwLock::new(HashMap::new()));
        let (broadcaster, _) = broadcast::channel(1000);

        Self {
            addr,
            clients,
            broadcaster,
        }
    }

    pub async fn start(&self) -> tokio::io::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        info!("Serveur WebSocket démarré sur {}", self.addr);

        while let Ok((stream, addr)) = listener.accept().await {
            info!("Nouvelle connexion depuis {}", addr);

            let clients = Arc::clone(&self.clients);
            let broadcaster = self.broadcaster.clone();

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, addr, clients, broadcaster).await {
                    error!("Erreur lors du traitement de la connexion {}: {}", addr, e);
                }
            });
        }

        Ok(())
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    clients: Clients,
    broadcaster: Broadcaster,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    let (ws_sender, mut ws_receiver) = ws_stream.split();
    let ws_sender = Arc::new(Mutex::new(ws_sender));

    // Attendre le message de connexion avec le nom d'utilisateur
    let username = match ws_receiver.next().await {
        Some(Ok(WsMessage::Text(text))) => {
            match Message::from_json(&text) {
                Ok(Message::UserJoin { user }) => user,
                _ => {
                    warn!("Premier message invalide de {}", addr);
                    return Ok(());
                }
            }
        }
        _ => {
            warn!("Pas de message de connexion de {}", addr);
            return Ok(());
        }
    };

    let client_info = ClientInfo::new(username.clone());
    let client_id = client_info.id.clone();

    // Ajouter le client à la liste
    {
        let mut clients_guard = clients.write().await;
        clients_guard.insert(client_id.clone(), client_info);
    }

    info!("Utilisateur '{}' connecté (ID: {})", username, client_id);
    println!("[DEBUG] Client '{}' connecté (ID: {})", username, client_id);

    // Notifier les autres clients
    let join_msg = Message::user_join(username.clone());
    if let Ok(_json) = join_msg.to_json() {
        let _ = broadcaster.send((client_id.clone(), join_msg));
    }

    // Créer un récepteur pour les broadcasts
    let mut receiver = broadcaster.subscribe();

    // Task pour envoyer les messages aux clients
    let _clients_clone = Arc::clone(&clients);
    let client_id_clone = client_id.clone();
    let ws_sender_broadcast = Arc::clone(&ws_sender);
    let broadcast_task = tokio::spawn(async move {
        while let Ok((sender_id, message)) = receiver.recv().await {
            // Ne pas renvoyer le message à l'expéditeur
            if sender_id == client_id_clone {
                continue;
            }

            if let Ok(json) = message.to_json() {
                let mut sender = ws_sender_broadcast.lock().await;
                if let Err(e) = sender.send(WsMessage::Text(json)).await {
                    error!("Erreur envoi message: {}", e);
                    break;
                }
            }
        }
    });

    // Task pour recevoir les messages du client
    let broadcaster_clone = broadcaster.clone();
    let client_id_clone2 = client_id.clone();
    let username_clone = username.clone();
    let ws_sender_receive = Arc::clone(&ws_sender);
    let receive_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(WsMessage::Text(text)) => {
                    debug!("Message reçu de {}: {}", username_clone, text);

                    match Message::from_json(&text) {
                        Ok(Message::Text { content, .. }) => {
                            let response = Message::text(content, username_clone.clone());
                            let _ = broadcaster_clone.send((client_id_clone2.clone(), response));
                        }
                        Ok(Message::Ping) => {
                            // Envoi direct du Pong à l'expéditeur
                            info!("Envoi du Pong à {}", username_clone);
                            if let Ok(json) = Message::Pong.to_json() {
                                let mut sender = ws_sender_receive.lock().await;
                                if let Err(e) = sender.send(WsMessage::Text(json)).await {
                                    error!("Erreur envoi Pong: {}", e);
                                } else {
                                    info!("Pong envoyé avec succès à {}", username_clone);
                                }
                            }
                        }
                        Ok(other_msg) => {
                            let _ = broadcaster_clone.send((client_id_clone2.clone(), other_msg));
                        }
                        Err(e) => {
                            warn!("Message JSON invalide de {}: {}", username_clone, e);
                        }
                    }
                }
                Ok(WsMessage::Binary(data)) => {
                    debug!("Données binaires reçues de {} ({} bytes)", username_clone, data.len());
                    let binary_msg = Message::binary(data, username_clone.clone());
                    let _ = broadcaster_clone.send((client_id_clone2.clone(), binary_msg));
                }
                Ok(WsMessage::Close(_)) => {
                    info!("Connexion fermée par {}", username_clone);
                    break;
                }
                Ok(WsMessage::Ping(_data)) => {
                    debug!("Ping reçu de {}", username_clone);
                    // Le ping/pong est géré automatiquement par tungstenite
                }
                Ok(WsMessage::Pong(_)) => {
                    debug!("Pong reçu de {}", username_clone);
                }
                Err(e) => {
                    error!("Erreur WebSocket de {}: {}", username_clone, e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Attendre qu'une des tâches se termine
    tokio::select! {
        _ = broadcast_task => {}
        _ = receive_task => {}
    }

    // Nettoyer lors de la déconnexion
    {
        let mut clients_guard = clients.write().await;
        clients_guard.remove(&client_id);
    }

    // Notifier la déconnexion
    let leave_msg = Message::user_leave(username);
    let _ = broadcaster.send((client_id, leave_msg));

    info!("Client déconnecté");
    Ok(())
}
