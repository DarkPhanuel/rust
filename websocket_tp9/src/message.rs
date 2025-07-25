use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
/// Représente les différents types de messages pouvant transiter sur le WebSocket.
pub enum Message {
    #[serde(rename = "text")]
    /// Message texte envoyé par un utilisateur.
    Text { content: String, user: String },

    #[serde(rename = "binary")]
    /// Message binaire envoyé par un utilisateur.
    Binary { data: Vec<u8>, user: String },

    #[serde(rename = "join")]
    /// Notification lorsqu'un utilisateur rejoint le chat.
    UserJoin { user: String },

    #[serde(rename = "leave")]
    /// Notification lorsqu'un utilisateur quitte le chat.
    UserLeave { user: String },

    #[serde(rename = "ping")]
    /// Message de ping pour vérifier la connexion.
    Ping,

    #[serde(rename = "pong")]
    /// Réponse à un ping.
    Pong,

    #[serde(rename = "error")]
    /// Message d'erreur envoyé par le serveur ou le client.
    Error { message: String },

    #[serde(rename = "system")]
    /// Message système pour informations diverses.
    System { message: String },
}

impl Message {
    pub fn text(content: String, user: String) -> Self {
        Message::Text { content, user }
    }

    pub fn binary(data: Vec<u8>, user: String) -> Self {
        Message::Binary { data, user }
    }

    pub fn user_join(user: String) -> Self {
        Message::UserJoin { user }
    }

    pub fn user_leave(user: String) -> Self {
        Message::UserLeave { user }
    }

    pub fn system(message: String) -> Self {
        Message::System { message }
    }

    pub fn error(message: String) -> Self {
        Message::Error { message }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[derive(Debug, Clone)]
/// Informations sur un client connecté au serveur WebSocket.
pub struct ClientInfo {
    /// Identifiant unique du client (UUID).
    pub id: String,
    /// Nom d'utilisateur du client.
    pub username: String,
    /// Date et heure de connexion du client.
    pub connected_at: std::time::SystemTime,
}

impl ClientInfo {
    pub fn new(username: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            connected_at: std::time::SystemTime::now(),
        }
    }
}
