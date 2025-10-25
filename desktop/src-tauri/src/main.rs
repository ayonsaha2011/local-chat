// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use commands::*;
use state::AppState;
use std::sync::Arc;
use tauri::Manager;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create app state
    let app_state = Arc::new(AppState::new());

    tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            initialize_app,
            get_user_profile,
            update_user_profile,
            get_peers,
            send_message,
            get_messages,
            send_file,
            accept_file_transfer,
            reject_file_transfer,
            get_file_transfers,
        ])
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            let state = app_state.clone();

            // Start services
            tokio::spawn(async move {
                if let Err(e) = state.start_services().await {
                    eprintln!("Failed to start services: {}", e);
                }
            });

            // Setup event listener
            let state_clone = app_state.clone();
            let window_clone = window.clone();
            tokio::spawn(async move {
                state_clone.listen_events(window_clone).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
