use std::sync::Mutex;

use crate::config::app_config::AppConfig;

pub struct AppState {
    pub app_config: Mutex<AppConfig>,
}

pub struct DbConnections {
    pub connections: Vec<u32>,
}

impl DbConnections {
    // TODO Add necessary functions here.
}
