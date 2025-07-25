use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, debug};
use url::Url;

use crate::message::Message;

pub struct WebSocketClient {
    server_url: String,
    username: String,
}

impl WebSocketClient {
    pub fn new(server_url: String, username: String) -> Self {
        Self {
            server_url,
            username,
        }
    }

    pub async fn connect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = Url::parse(&self.server_url)?;
        info!("Connexion à {}", url);

        let (ws_stream, _) = connect_async(url).await?;
        let (ws_sender, mut ws_receiver) = ws_stream.split();
        let ws_sender = Arc::new(Mutex::new(ws_sender));

        // Envoyer le message de connexion
        let join_msg = Message::user_join(self.username.clone());
        let join_json = join_msg.to_json()?;
        {
            let mut sender = ws_sender.lock().await;
            if let Err(e) = sender.send(WsMessage::Text(join_json)).await {
                error!("Erreur envoi message: {}", e);
                return Err(Box::new(e));
            }
        }

        info!("Connecté en tant que '{}'", self.username);
        println!("=== Chat WebSocket ===");
        println!("Tapez vos messages et appuyez sur Entrée");
        println!("Commandes spéciales:");
        println!("  /ping - Envoyer un ping");
        println!("  /quit - Quitter");
        println!("======================");

        // Task pour lire l'input utilisateur (lancée AVANT la tâche de réception)
        let ws_sender_clone = Arc::clone(&ws_sender);
        let username_clone = self.username.clone();
        let input_task = tokio::spawn(async move {
            let stdin = tokio::io::stdin();
            let reader = BufReader::new(stdin);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                let message = match line {
                    "/quit" => break,
                    "/ping" => {
                        println!("[DEBUG] Envoi du message Ping au serveur");
                        Message::Ping
                    },
                    input if input.starts_with("/") => {
                        println!("Commande inconnue: {}", input);
                        continue;
                    }
                    input => {
                        println!("[DEBUG] Envoi du message texte: {}", input);
                        Message::text(input.to_string(), username_clone.clone())
                    },
                };

                if let Ok(json) = message.to_json() {
                    let mut sender = ws_sender_clone.lock().await;
                    let json_clone = json.clone();
                    match sender.send(WsMessage::Text(json)).await {
                        Ok(_) => {
                            println!("[DEBUG] Message envoyé au serveur: {}", json_clone);
                        },
                        Err(e) => {
                            error!("Erreur envoi message: {}", e);
                            println!("[DEBUG] Erreur envoi message: {}", e);
                            if e.to_string().contains("connection closed") {
                                println!("[DEBUG] La connexion WebSocket est fermée côté client");
                            }
                            break;
                        }
                    }
                } else {
                    println!("[DEBUG] Erreur de sérialisation du message");
                }
            }
        });

        // Task pour recevoir les messages du serveur
        let receive_task = tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(WsMessage::Text(text)) => {
                        println!("[DEBUG] Message brut reçu: {}", text);
                        match Message::from_json(&text) {
                            Ok(Message::Text { content, user }) => {
                                println!("[{}]: {}", user, content);
                            }
                            Ok(Message::UserJoin { user }) => {
                                println!("*** {} a rejoint le chat ***", user);
                            }
                            Ok(Message::UserLeave { user }) => {
                                println!("*** {} a quitté le chat ***", user);
                            }
                            Ok(Message::System { message }) => {
                                println!("SYSTÈME: {}", message);
                            }
                            Ok(Message::Error { message }) => {
                                println!("ERREUR: {}", message);
                            }
                            Ok(Message::Pong) => {
                                println!("Pong reçu du serveur");
                            }
                            Ok(Message::Binary { user, data }) => {
                                println!("[{}] a envoyé {} bytes de données binaires", user, data.len());
                            }
                            Ok(_) => {
                                println!("[DEBUG] Message JSON non géré: {}", text);
                                debug!("Message non géré: {}", text);
                            }
                            Err(e) => {
                                error!("Erreur parsing JSON: {}", e);
                                println!("[DEBUG] Erreur parsing JSON: {}", e);
                            }
                        }
                    }
                    Ok(WsMessage::Binary(data)) => {
                        println!("Données binaires reçues: {} bytes", data.len());
                    }
                    Ok(WsMessage::Close(_)) => {
                        println!("Connexion fermée par le serveur");
                        println!("[DEBUG] Connexion WebSocket fermée (Close)");
                        break;
                    }
                    Ok(WsMessage::Ping(_)) => {
                        debug!("Ping reçu du serveur");
                    }
                    Ok(WsMessage::Pong(_)) => {
                        debug!("Pong reçu du serveur");
                    }
                    Ok(WsMessage::Frame(_)) => {
                        // Frame non gérée explicitement (cas rare)
                        debug!("Frame WebSocket non gérée reçue");
                    }
                    Err(e) => {
                        error!("Erreur WebSocket: {}", e);
                        break;
                    }
                }
            }
        });

        // Attendre qu'une des tâches se termine
        tokio::select! {
            _ = input_task => {}
            _ = receive_task => {}
        }

        println!("Déconnexion...");
        Ok(())
    }
}
