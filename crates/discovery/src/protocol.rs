use lan_chat_core::{NetworkAddress, UserProfile};
use serde::{Deserialize, Serialize};

/// Multicast port for peer discovery
pub const DISCOVERY_PORT: u16 = 37842;

/// Multicast address for IPv4
pub const MULTICAST_ADDR_V4: &str = "239.255.42.99";

/// Multicast address for IPv6
pub const MULTICAST_ADDR_V6: &str = "ff02::1";

/// Discovery protocol messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMessage {
    /// Announce presence on the network
    Announce {
        profile: UserProfile,
        address: NetworkAddress,
        public_key: Option<Vec<u8>>,
    },

    /// Request all peers to announce themselves
    DiscoveryRequest,

    /// Response to discovery request
    DiscoveryResponse {
        profile: UserProfile,
        address: NetworkAddress,
        public_key: Option<Vec<u8>>,
    },

    /// Announce going offline
    Goodbye {
        user_id: uuid::Uuid,
    },

    /// Heartbeat to maintain presence
    Heartbeat {
        user_id: uuid::Uuid,
        status: lan_chat_core::UserStatus,
    },
}

impl DiscoveryMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}
