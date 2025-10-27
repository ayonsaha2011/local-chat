use crate::{NetworkAddress, UserProfile, UserId, UserStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents a peer in the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub profile: UserProfile,
    pub address: NetworkAddress,
    pub last_seen: DateTime<Utc>,
    pub public_key: Option<Vec<u8>>,
}

impl Peer {
    pub fn new(profile: UserProfile, address: NetworkAddress) -> Self {
        Self {
            profile,
            address,
            last_seen: Utc::now(),
            public_key: None,
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }

    pub fn is_online(&self) -> bool {
        // Mark as online if last seen within 30 seconds (2 missed heartbeats at 15s interval)
        // Peers are cleaned up after 45 seconds (see PEER_TIMEOUT in discovery service)
        const ONLINE_THRESHOLD_SECONDS: i64 = 30;
        self.profile.status != UserStatus::Offline
            && Utc::now().signed_duration_since(self.last_seen).num_seconds() < ONLINE_THRESHOLD_SECONDS
    }
}

/// Thread-safe peer registry
#[derive(Debug, Clone)]
pub struct PeerRegistry {
    peers: Arc<RwLock<HashMap<UserId, Peer>>>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_peer(&self, peer: Peer) {
        let mut peers = self.peers.write().await;
        peers.insert(peer.profile.user_id, peer);
    }

    pub async fn remove_peer(&self, user_id: &UserId) -> Option<Peer> {
        let mut peers = self.peers.write().await;
        peers.remove(user_id)
    }

    pub async fn get_peer(&self, user_id: &UserId) -> Option<Peer> {
        let peers = self.peers.read().await;
        peers.get(user_id).cloned()
    }

    pub async fn update_peer_status(&self, user_id: &UserId, status: UserStatus) {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(user_id) {
            peer.profile.status = status;
            peer.update_last_seen();
        }
    }

    pub async fn get_all_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    pub async fn get_online_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().await;
        peers
            .values()
            .filter(|p| p.is_online())
            .cloned()
            .collect()
    }

    pub async fn cleanup_offline_peers(&self, timeout_seconds: i64) {
        let mut peers = self.peers.write().await;
        let now = Utc::now();
        peers.retain(|_, peer| {
            now.signed_duration_since(peer.last_seen).num_seconds() < timeout_seconds
        });
    }
}

impl Default for PeerRegistry {
    fn default() -> Self {
        Self::new()
    }
}
