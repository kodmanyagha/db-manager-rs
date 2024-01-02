use anyhow::Result;

pub struct AppConfig {
    pub window_left: u32,
    pub window_top: u32,
    pub window_width: u32,
    pub window_height: u32,
}

pub fn detect_config() -> Result<AppConfig> {
    // TODO Detect all configs from configuration file.

    Ok(AppConfig {
        window_left: 10,
        window_top: 10,
        window_width: 500,
        window_height: 500,
    })
}
