use crate::state::AppState;
use lan_chat_core::{Message, Peer, UserProfile};
use lan_chat_transfer::FileTransfer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeRequest {
    pub username: String,
    pub display_name: String,
}

#[tauri::command]
pub async fn initialize_app(
    state: State<'_, Arc<AppState>>,
    request: InitializeRequest,
) -> Result<UserProfile, String> {
    let profile = UserProfile::new(request.username, request.display_name);
    *state.user_profile.write().await = Some(profile.clone());
    Ok(profile)
}

#[tauri::command]
pub async fn get_user_profile(
    state: State<'_, Arc<AppState>>,
) -> Result<Option<UserProfile>, String> {
    Ok(state.user_profile.read().await.clone())
}

#[tauri::command]
pub async fn update_user_profile(
    state: State<'_, Arc<AppState>>,
    profile: UserProfile,
) -> Result<(), String> {
    *state.user_profile.write().await = Some(profile);
    Ok(())
}

#[tauri::command]
pub async fn get_peers(state: State<'_, Arc<AppState>>) -> Result<Vec<Peer>, String> {
    Ok(state.peer_registry.get_all_peers().await)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub recipient_id: String,
    pub content: String,
}

#[tauri::command]
pub async fn send_message(
    state: State<'_, Arc<AppState>>,
    request: SendMessageRequest,
) -> Result<Message, String> {
    let profile = state
        .user_profile
        .read()
        .await
        .clone()
        .ok_or("Not initialized")?;

    let recipient_id = Uuid::parse_str(&request.recipient_id)
        .map_err(|e| format!("Invalid recipient ID: {}", e))?;

    let message = Message::new_text(
        Uuid::new_v4(),
        profile.user_id,
        recipient_id,
        request.content,
    );

    // TODO: Send via messaging server
    let mut messages = state.messages.write().await;
    messages.push(message.clone());

    Ok(message)
}

#[tauri::command]
pub async fn get_messages(state: State<'_, Arc<AppState>>) -> Result<Vec<Message>, String> {
    Ok(state.messages.read().await.clone())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendFileRequest {
    pub recipient_id: String,
    pub file_path: String,
}

#[tauri::command]
pub async fn send_file(
    _state: State<'_, Arc<AppState>>,
    request: SendFileRequest,
) -> Result<String, String> {
    let _recipient_id = Uuid::parse_str(&request.recipient_id)
        .map_err(|e| format!("Invalid recipient ID: {}", e))?;

    // TODO: Implement via transfer service
    Ok("transfer-id".to_string())
}

#[tauri::command]
pub async fn accept_file_transfer(
    _state: State<'_, Arc<AppState>>,
    _transfer_id: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn reject_file_transfer(
    _state: State<'_, Arc<AppState>>,
    _transfer_id: String,
) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[tauri::command]
pub async fn get_file_transfers(
    _state: State<'_, Arc<AppState>>,
) -> Result<Vec<FileTransfer>, String> {
    // TODO: Implement
    Ok(Vec::new())
}
