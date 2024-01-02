// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod config;
pub mod db;
pub mod handler;
pub mod state;

use std::sync::Mutex;

use anyhow::Result;
use config::app_config::detect_config;
use handler::invoke_handler::invoke_handler;
use state::app_state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    let app_config = detect_config()?;

    tauri::Builder::default()
        .manage(AppState {
            app_config: Mutex::new(app_config),
        })
        .invoke_handler(tauri::generate_handler![invoke_handler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
