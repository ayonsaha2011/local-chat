use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use uuid::Uuid;

/// User identifier
pub type UserId = Uuid;

/// Session identifier
pub type SessionId = Uuid;

/// Transfer identifier for file transfers
pub type TransferId = Uuid;

/// Network address information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NetworkAddress {
    pub ip: IpAddr,
    pub port: u16,
}

impl NetworkAddress {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self { ip, port }
    }

    pub fn to_socket_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::new(self.ip, self.port)
    }
}

/// User status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserStatus {
    Online,
    Away,
    Busy,
    Offline,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Online
    }
}

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: UserId,
    pub username: String,
    pub display_name: String,
    pub status: UserStatus,
    pub status_message: Option<String>,
    pub avatar_hash: Option<String>,
}

impl UserProfile {
    pub fn new(username: String, display_name: String) -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username,
            display_name,
            status: UserStatus::Online,
            status_message: None,
            avatar_hash: None,
        }
    }
}
