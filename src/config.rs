use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pianobar: PianobarConfig,
}

#[derive(Debug, Deserialize)]
pub struct PianobarConfig {
    pub executable: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let contents = fs::read_to_string("config.toml")
            .context("failed to read config.toml")?;

        let config: Self =
            toml::from_str(&contents).context("failed to parse config.toml")?;

        Ok(config)
    }
}