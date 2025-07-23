use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_password: String,
}

impl Config {
    pub fn load(path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(path) = path {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            return Ok(config);
        }

        let home_config = std::env::var("HOME").ok().map(|home| {
            let mut path = PathBuf::from(home);
            path.push(".config/mtapi.toml");
            path
        });

        let fallback = PathBuf::from("mtapi.toml");

        let candidates = [
            home_config.as_ref().map(PathBuf::as_path),
            Some(fallback.as_path()),
        ];

        for path in candidates.into_iter().flatten() {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                let config: Config = toml::from_str(&content)?;
                return Ok(config);
            }
        }

        Err("No config file found in ~/.config/mtapi.toml or ./mtapi.toml".into())
    }

    pub fn base_url(&self) -> String {
        format!("{}:{}", self.host.trim_end_matches('/'), self.port)
    }
}
