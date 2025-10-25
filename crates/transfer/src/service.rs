use crate::protocol::{FileTransfer, TransferMessage, TransferStatus};
use crate::TRANSFER_PORT;
use lan_chat_core::{ChatEvent, PeerRegistry, TransferId, UserId};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};

const CHUNK_SIZE: usize = 64 * 1024; // 64 KB chunks

/// File transfer service
pub struct TransferService {
    user_id: UserId,
    peer_registry: PeerRegistry,
    transfers: Arc<RwLock<HashMap<TransferId, FileTransfer>>>,
    event_tx: mpsc::UnboundedSender<ChatEvent>,
    download_dir: PathBuf,
}

impl TransferService {
    pub fn new(
        user_id: UserId,
        peer_registry: PeerRegistry,
        event_tx: mpsc::UnboundedSender<ChatEvent>,
        download_dir: PathBuf,
    ) -> Self {
        Self {
            user_id,
            peer_registry,
            transfers: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            download_dir,
        }
    }

    /// Start the transfer service
    pub async fn start(self: Arc<Self>) -> lan_chat_core::Result<()> {
        let addr = SocketAddr::from(([0, 0, 0, 0], TRANSFER_PORT));
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        info!("Transfer service listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    debug!("New transfer connection from {}", addr);
                    let service = Arc::clone(&self);
                    tokio::spawn(async move {
                        if let Err(e) = service.handle_connection(stream).await {
                            error!("Transfer error from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting transfer connection: {}", e);
                }
            }
        }
    }

    /// Handle incoming transfer connection
    async fn handle_connection(&self, mut stream: TcpStream) -> lan_chat_core::Result<()> {
        // Read message length
        let length = stream
            .read_u32()
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        // Read message
        let mut buffer = vec![0u8; length as usize];
        stream
            .read_exact(&mut buffer)
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        let message = TransferMessage::from_bytes(&buffer)
            .map_err(|e| lan_chat_core::ChatError::Protocol(e.to_string()))?;

        match message {
            TransferMessage::TransferRequest {
                transfer_id,
                sender_id,
                file_name,
                file_size,
                file_hash,
            } => {
                // Store transfer info
                let transfer = FileTransfer::new(
                    sender_id,
                    self.user_id,
                    file_name.clone(),
                    file_size,
                    file_hash,
                );

                {
                    let mut transfers = self.transfers.write().await;
                    transfers.insert(transfer_id, transfer);
                }

                // Notify user about incoming transfer
                let _ = self.event_tx.send(ChatEvent::FileTransferRequested {
                    transfer_id,
                    from: sender_id,
                    file_name,
                    file_size,
                });

                // Wait for user decision (handled separately)
            }

            TransferMessage::StartTransfer { transfer_id } => {
                self.receive_file(stream, transfer_id).await?;
            }

            _ => {
                warn!("Unexpected transfer message: {:?}", message);
            }
        }

        Ok(())
    }

    /// Send a file to a peer
    pub async fn send_file(
        &self,
        recipient_id: UserId,
        file_path: &Path,
    ) -> lan_chat_core::Result<TransferId> {
        // Get file info
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| lan_chat_core::ChatError::FileTransfer("Invalid file name".into()))?
            .to_string();

        let metadata = tokio::fs::metadata(file_path)
            .await
            .map_err(|e| lan_chat_core::ChatError::FileTransfer(e.to_string()))?;

        let file_size = metadata.len();

        // Calculate file hash
        let file_hash = self.calculate_file_hash(file_path).await?;

        // Create transfer
        let transfer = FileTransfer::new(
            self.user_id,
            recipient_id,
            file_name.clone(),
            file_size,
            file_hash.clone(),
        );

        let transfer_id = transfer.transfer_id;

        {
            let mut transfers = self.transfers.write().await;
            transfers.insert(transfer_id, transfer);
        }

        // Get peer address
        let peer = self
            .peer_registry
            .get_peer(&recipient_id)
            .await
            .ok_or_else(|| lan_chat_core::ChatError::PeerNotFound(recipient_id.to_string()))?;

        let addr = peer.address.to_socket_addr();
        let transfer_addr = SocketAddr::new(addr.ip(), TRANSFER_PORT);

        // Connect and send request
        let mut stream = TcpStream::connect(transfer_addr)
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        let request = TransferMessage::TransferRequest {
            transfer_id,
            sender_id: self.user_id,
            file_name,
            file_size,
            file_hash,
        };

        self.send_message(&mut stream, &request).await?;

        // Wait for acceptance (handled separately)

        Ok(transfer_id)
    }

    /// Accept a file transfer
    pub async fn accept_transfer(&self, transfer_id: TransferId) -> lan_chat_core::Result<()> {
        // Update transfer status
        {
            let mut transfers = self.transfers.write().await;
            if let Some(transfer) = transfers.get_mut(&transfer_id) {
                transfer.status = TransferStatus::Accepted;
            } else {
                return Err(lan_chat_core::ChatError::FileTransfer(
                    "Transfer not found".into(),
                ));
            }
        }

        let _ = self.event_tx.send(ChatEvent::FileTransferAccepted { transfer_id });

        Ok(())
    }

    /// Reject a file transfer
    pub async fn reject_transfer(
        &self,
        transfer_id: TransferId,
        reason: String,
    ) -> lan_chat_core::Result<()> {
        {
            let mut transfers = self.transfers.write().await;
            if let Some(transfer) = transfers.get_mut(&transfer_id) {
                transfer.status = TransferStatus::Cancelled;
                transfer.error = Some(reason);
            }
        }

        Ok(())
    }

    /// Start sending file after acceptance
    pub async fn start_sending(
        &self,
        transfer_id: TransferId,
        file_path: &Path,
    ) -> lan_chat_core::Result<()> {
        let transfer = {
            let transfers = self.transfers.read().await;
            transfers
                .get(&transfer_id)
                .cloned()
                .ok_or_else(|| lan_chat_core::ChatError::FileTransfer("Transfer not found".into()))?
        };

        // Get recipient address
        let peer = self
            .peer_registry
            .get_peer(&transfer.recipient_id)
            .await
            .ok_or_else(|| {
                lan_chat_core::ChatError::PeerNotFound(transfer.recipient_id.to_string())
            })?;

        let addr = peer.address.to_socket_addr();
        let transfer_addr = SocketAddr::new(addr.ip(), TRANSFER_PORT);

        // Connect
        let mut stream = TcpStream::connect(transfer_addr)
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        // Send start message
        let start_msg = TransferMessage::StartTransfer { transfer_id };
        self.send_message(&mut stream, &start_msg).await?;

        // Update status
        {
            let mut transfers = self.transfers.write().await;
            if let Some(transfer) = transfers.get_mut(&transfer_id) {
                transfer.status = TransferStatus::InProgress;
            }
        }

        // Open file
        let mut file = File::open(file_path)
            .await
            .map_err(|e| lan_chat_core::ChatError::FileTransfer(e.to_string()))?;

        // Send file in chunks
        let mut chunk_index = 0u64;
        let mut buffer = vec![0u8; CHUNK_SIZE];

        loop {
            let bytes_read = file
                .read(&mut buffer)
                .await
                .map_err(|e| lan_chat_core::ChatError::FileTransfer(e.to_string()))?;

            if bytes_read == 0 {
                break;
            }

            let chunk = TransferMessage::DataChunk {
                transfer_id,
                chunk_index,
                data: buffer[..bytes_read].to_vec(),
            };

            self.send_message(&mut stream, &chunk).await?;

            // Update progress
            {
                let mut transfers = self.transfers.write().await;
                if let Some(transfer) = transfers.get_mut(&transfer_id) {
                    transfer.bytes_transferred += bytes_read as u64;

                    let _ = self.event_tx.send(ChatEvent::FileTransferProgress {
                        transfer_id,
                        bytes_transferred: transfer.bytes_transferred,
                        total_bytes: transfer.file_size,
                    });
                }
            }

            chunk_index += 1;
        }

        // Send completion
        let complete = TransferMessage::TransferComplete { transfer_id };
        self.send_message(&mut stream, &complete).await?;

        // Update status
        {
            let mut transfers = self.transfers.write().await;
            if let Some(transfer) = transfers.get_mut(&transfer_id) {
                transfer.status = TransferStatus::Completed;
            }
        }

        let _ = self
            .event_tx
            .send(ChatEvent::FileTransferCompleted { transfer_id });

        Ok(())
    }

    /// Receive a file
    async fn receive_file(
        &self,
        mut stream: TcpStream,
        transfer_id: TransferId,
    ) -> lan_chat_core::Result<()> {
        let (file_name, file_size) = {
            let transfers = self.transfers.read().await;
            let transfer = transfers.get(&transfer_id).ok_or_else(|| {
                lan_chat_core::ChatError::FileTransfer("Transfer not found".into())
            })?;

            (transfer.file_name.clone(), transfer.file_size)
        };

        // Create file
        let file_path = self.download_dir.join(&file_name);
        let mut file = File::create(&file_path)
            .await
            .map_err(|e| lan_chat_core::ChatError::FileTransfer(e.to_string()))?;

        // Update status
        {
            let mut transfers = self.transfers.write().await;
            if let Some(transfer) = transfers.get_mut(&transfer_id) {
                transfer.status = TransferStatus::InProgress;
            }
        }

        // Receive chunks
        loop {
            let length = stream
                .read_u32()
                .await
                .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

            let mut buffer = vec![0u8; length as usize];
            stream
                .read_exact(&mut buffer)
                .await
                .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

            let message = TransferMessage::from_bytes(&buffer)
                .map_err(|e| lan_chat_core::ChatError::Protocol(e.to_string()))?;

            match message {
                TransferMessage::DataChunk { data, .. } => {
                    file.write_all(&data)
                        .await
                        .map_err(|e| lan_chat_core::ChatError::FileTransfer(e.to_string()))?;

                    // Update progress
                    {
                        let mut transfers = self.transfers.write().await;
                        if let Some(transfer) = transfers.get_mut(&transfer_id) {
                            transfer.bytes_transferred += data.len() as u64;

                            let _ = self.event_tx.send(ChatEvent::FileTransferProgress {
                                transfer_id,
                                bytes_transferred: transfer.bytes_transferred,
                                total_bytes: transfer.file_size,
                            });
                        }
                    }
                }

                TransferMessage::TransferComplete { .. } => {
                    file.flush()
                        .await
                        .map_err(|e| lan_chat_core::ChatError::FileTransfer(e.to_string()))?;

                    // Update status
                    {
                        let mut transfers = self.transfers.write().await;
                        if let Some(transfer) = transfers.get_mut(&transfer_id) {
                            transfer.status = TransferStatus::Completed;
                        }
                    }

                    let _ = self
                        .event_tx
                        .send(ChatEvent::FileTransferCompleted { transfer_id });

                    break;
                }

                TransferMessage::TransferFailed { error, .. } => {
                    {
                        let mut transfers = self.transfers.write().await;
                        if let Some(transfer) = transfers.get_mut(&transfer_id) {
                            transfer.status = TransferStatus::Failed;
                            transfer.error = Some(error.clone());
                        }
                    }

                    let _ = self.event_tx.send(ChatEvent::FileTransferFailed {
                        transfer_id,
                        error,
                    });

                    break;
                }

                _ => {}
            }
        }

        Ok(())
    }

    /// Calculate SHA-256 hash of a file
    async fn calculate_file_hash(&self, file_path: &Path) -> lan_chat_core::Result<String> {
        let mut file = File::open(file_path)
            .await
            .map_err(|e| lan_chat_core::ChatError::FileTransfer(e.to_string()))?;

        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; CHUNK_SIZE];

        loop {
            let bytes_read = file
                .read(&mut buffer)
                .await
                .map_err(|e| lan_chat_core::ChatError::FileTransfer(e.to_string()))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        Ok(hex::encode(hasher.finalize()))
    }

    /// Send a transfer message
    async fn send_message(
        &self,
        stream: &mut TcpStream,
        message: &TransferMessage,
    ) -> lan_chat_core::Result<()> {
        let data = message
            .to_bytes()
            .map_err(|e| lan_chat_core::ChatError::Protocol(e.to_string()))?;

        let length = data.len() as u32;

        stream
            .write_u32(length)
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        stream
            .write_all(&data)
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        stream
            .flush()
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        Ok(())
    }
}
