use anyhow::Result;
use thiserror::Error;

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

pub async fn mysql_connect(dsn: String, window: tauri::Window) -> Result<String> {
    window.eval("alert('Selaminko aleykümko aga')")?;

    Ok(format!("DSN received: {}", dsn))
}
