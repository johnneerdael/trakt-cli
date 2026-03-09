use std::{fs, path::PathBuf};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// Configuration for the Trakt CLI.
/// Stored as JSON in the platform‑specific configuration directory.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// Client ID from Trakt developer portal.
    pub client_id: String,
    /// Client secret from Trakt developer portal.
    pub client_secret: String,
    /// OAuth access token (valid for 7 days).
    pub access_token: Option<String>,
    /// OAuth refresh token.
    pub refresh_token: Option<String>,
}

impl Config {
    /// Determine the path to the config file, creating parent directories as needed.
    fn config_path() -> anyhow::Result<PathBuf> {
        let proj = ProjectDirs::from("com", "trakt", "trakt-cli")
            .ok_or_else(|| anyhow::anyhow!("Unable to determine configuration directory"))?;
        let cfg_dir = proj.config_dir();
        fs::create_dir_all(cfg_dir)?;
        Ok(cfg_dir.join("config.json"))
    }

    /// Load configuration from disk, returning defaults if the file does not exist.
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let data = fs::read_to_string(&path)?;
            let cfg: Config = serde_json::from_str(&data)?;
            Ok(cfg)
        } else {
            Ok(Config::default())
        }
    }

    /// Save the current configuration to disk.
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path()?;
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}

