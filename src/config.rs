use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub openai_api_key: Option<String>,
}

impl Config {
    /// Returns ~/.config/aihub/ and ensures it exists.
    pub fn config_dir() -> Result<PathBuf> {
        let home = std::env::var("HOME").context("HOME environment variable not set")?;

        let config_dir = PathBuf::from(home).join(".config").join("aihub");

        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory (~/.config/aihub)")?;

        Ok(config_dir)
    }

    /// Returns ~/.config/aihub/config.toml
    pub fn config_file() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Save the config to config.toml
    pub fn save(&self) -> Result<()> {
        let config_file = Self::config_file()?;

        let contents =
            toml::to_string_pretty(self).context("Failed to serialize config to TOML")?;

        fs::write(&config_file, contents).context("Failed to write config.toml")?;

        Ok(())
    }

    /// Set the API key (mutates) and persist.
    pub fn set_api_key(&mut self, key: String) -> Result<()> {
        self.openai_api_key = Some(key);
        self.save()?;
        println!("✅ API key saved to {}", Self::config_file()?.display());
        Ok(())
    }

    /// Load the config from config.toml and require an API key.
    /// If config doesn't exist, create a default file and instruct the user.
    pub fn load() -> Result<Self> {
        let config_file = Self::config_file()?;

        if !config_file.exists() {
            let empty = Config::default();
            empty
                .save()
                .context("Failed to write default config.toml")?;

            bail!(
                "❌ No API key found.\n\
                 A new config file has been created at:\n\
                 {}\n\
                 Please add your key using:\n\
                 aihub config set-key YOUR_KEY",
                config_file.display()
            );
        }

        let contents = fs::read_to_string(&config_file).context("Failed to read config.toml")?;

        let config: Self = toml::from_str(&contents).context("Failed to parse config.toml")?;

        match &config.openai_api_key {
            Some(k) if !k.trim().is_empty() => Ok(config),
            _ => bail!(
                "❌ No API key set in config.toml.\n\
                 Edit the file or run:\n\
                 aihub config set-key YOUR_KEY"
            ),
        }
    }
}
