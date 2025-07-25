// =============================
// Protocole RustChat
// Format d'un message :
// [4 octets: longueur totale (u32, big-endian)] [1 octet: type de message (u8)] [données sérialisées JSON]
// =============================

pub mod client;
pub mod server;

use serde::{Deserialize, Serialize};
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Types de messages du protocole
pub const MSG_CONNECT: u8 = 0x01;
pub const MSG_PUBLIC_MESSAGE: u8 = 0x02;
pub const MSG_PRIVATE_MESSAGE: u8 = 0x03;
pub const MSG_LIST_USERS: u8 = 0x04;
pub const MSG_DISCONNECT: u8 = 0x05;

pub const MSG_CONNECT_RESPONSE: u8 = 0x10;
pub const MSG_MESSAGE_BROADCAST: u8 = 0x11;
pub const MSG_PRIVATE_MESSAGE_DELIVERY: u8 = 0x12;
pub const MSG_USER_LIST: u8 = 0x13;
pub const MSG_USER_JOINED: u8 = 0x14;
pub const MSG_USER_LEFT: u8 = 0x15;
pub const MSG_ERROR: u8 = 0x16;

// Messages Client -> Serveur
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Connect { username: String },
    PublicMessage { content: String },
    PrivateMessage { to: String, content: String },
    ListUsers,
    Disconnect,
}

// Messages Serveur -> Client
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    ConnectResponse { success: bool, message: String },
    MessageBroadcast { from: String, content: String },
    PrivateMessageDelivery { from: String, content: String },
    UserList { users: Vec<String> },
    UserJoined { username: String },
    UserLeft { username: String },
    Error { message: String },
}

// Structure d'un message protocolaire
#[derive(Debug)]
pub struct ProtocolMessage {
    pub msg_type: u8,
    pub data: Vec<u8>,
}

impl ProtocolMessage {
    // Crée un nouveau message protocolaire
    pub fn new(msg_type: u8, data: Vec<u8>) -> Self {
        Self { msg_type, data }
    }

    // Sérialise le message en bytes pour transmission
    pub fn serialize(&self) -> Vec<u8> {
        let total_len = 5 + self.data.len(); // 4 bytes longueur + 1 byte type + data
        let mut buffer = Vec::with_capacity(total_len);

        // Longueur totale (big-endian)
        buffer.extend_from_slice(&(total_len as u32).to_be_bytes());
        // Type de message
        buffer.push(self.msg_type);
        // Données
        buffer.extend_from_slice(&self.data);

        buffer
    }

    // Lit un message depuis un stream TCP
    pub async fn read_from<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Self> {
        // Lit la longueur totale
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf).await?;
        let total_len = u32::from_be_bytes(len_buf) as usize;

        if total_len < 5 || total_len > 1024 * 1024 { // Max 1MB
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Message size invalid"
            ));
        }

        // Lit le type de message
        let msg_type = reader.read_u8().await?;

        // Lit les données
        let data_len = total_len - 5;
        let mut data = vec![0u8; data_len];
        reader.read_exact(&mut data).await?;

        Ok(ProtocolMessage::new(msg_type, data))
    }

    // Écrit un message sur un stream TCP
    pub async fn write_to<W: AsyncWriteExt + Unpin>(&self, writer: &mut W) -> io::Result<()> {
        let serialized = self.serialize();
        writer.write_all(&serialized).await?;
        writer.flush().await?;
        Ok(())
    }
}

// Utilitaires pour les messages client
impl ClientMessage {
    pub fn to_protocol_message(&self) -> anyhow::Result<ProtocolMessage> {
        let data = serde_json::to_vec(self)?;
        let msg_type = match self {
            ClientMessage::Connect { .. } => MSG_CONNECT,
            ClientMessage::PublicMessage { .. } => MSG_PUBLIC_MESSAGE,
            ClientMessage::PrivateMessage { .. } => MSG_PRIVATE_MESSAGE,
            ClientMessage::ListUsers => MSG_LIST_USERS,
            ClientMessage::Disconnect => MSG_DISCONNECT,
        };
        Ok(ProtocolMessage::new(msg_type, data))
    }

    pub fn from_protocol_message(msg: &ProtocolMessage) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(&msg.data)?)
    }
}

// Utilitaires pour les messages serveur
impl ServerMessage {
    pub fn to_protocol_message(&self) -> anyhow::Result<ProtocolMessage> {
        let data = serde_json::to_vec(self)?;
        let msg_type = match self {
            ServerMessage::ConnectResponse { .. } => MSG_CONNECT_RESPONSE,
            ServerMessage::MessageBroadcast { .. } => MSG_MESSAGE_BROADCAST,
            ServerMessage::PrivateMessageDelivery { .. } => MSG_PRIVATE_MESSAGE_DELIVERY,
            ServerMessage::UserList { .. } => MSG_USER_LIST,
            ServerMessage::UserJoined { .. } => MSG_USER_JOINED,
            ServerMessage::UserLeft { .. } => MSG_USER_LEFT,
            ServerMessage::Error { .. } => MSG_ERROR,
        };
        Ok(ProtocolMessage::new(msg_type, data))
    }

    pub fn from_protocol_message(msg: &ProtocolMessage) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(&msg.data)?)
    }
}

// États de session
#[derive(Debug, Clone)]
pub enum ClientState {
    Disconnected,
    Connected(String), // username
}

#[derive(Debug, Clone)]
pub enum ServerClientState {
    WaitingAuth,
    Authenticated(String), // username
}
