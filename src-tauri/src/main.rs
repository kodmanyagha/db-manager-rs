// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod handler;

use anyhow::Result;
use handler::invoke_handler::invoke_handler;

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![invoke_handler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
