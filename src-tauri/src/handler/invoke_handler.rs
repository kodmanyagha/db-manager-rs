use crate::db::mysql::connection::mysql_connect;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum InvokeHandlerError {
    #[error("Function not found.")]
    FuncNotFound,

    #[error("General error.")]
    GeneralError(String),
}

#[tauri::command]
pub async fn invoke_handler(
    func: String,
    data: String,
    window: tauri::Window,
    app_handle: tauri::AppHandle,
) -> Result<impl Serialize, String> {
    let result = _invoke_handler(func, data, window, app_handle).await;

    match result {
        Ok(data) => Ok(data.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

async fn _invoke_handler(
    func: String,
    data: String,
    window: tauri::Window,
    #[allow(unused_variables)] app_handle: tauri::AppHandle,
) -> anyhow::Result<String> {
    match func.as_str() {
        "mysql_connect" => mysql_connect(data, window).await,
        "example_json" => example_json(data).await,
        _ => Err(InvokeHandlerError::FuncNotFound.into()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExampleStruct {
    pub u1: u32,
    pub i1: i32,
    pub s1: String,
}

async fn example_json(data: String) -> anyhow::Result<String> {
    let result = serde_json::from_str::<ExampleStruct>(data.as_str())?;
    println!(">> result: {:?}", result);

    Ok("test".into())
}
