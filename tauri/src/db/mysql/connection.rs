use anyhow::Result;
use thiserror::Error;

use crate::state::app_state::AppState;

pub struct Mysql {
    //
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Wrond credentials.")]
    WrondCredentials,

    #[error("Remote server not running.")]
    ServerNotRunning,

    #[error("General error.")]
    GeneralError(String),
}

pub async fn mysql_connect(
    dsn: String,
    window: tauri::Window,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String> {
    app.show()?;

    let app_config = state
        .app_config
        .lock()
        .map_err(|_| ConnectionError::GeneralError("app_config can not be locked.".to_string()))?;

    println!("App height: {}", app_config.window_height);

    window.eval("alert('Selaminko aleykümko aga')")?;

    Ok(format!("DSN received: {}", dsn))
}
