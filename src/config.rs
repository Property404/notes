use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn default_config_path() -> PathBuf {
    Path::new(&env::var("HOME").expect("$HOME not set"))
        .join(".config/dev.dagans.notes/config.toml")
}

fn default_repo_path() -> PathBuf {
    Path::new(&env::var("HOME").expect("$HOME not set")).join(".local/share/dev.dagans.notes")
}

/// Global configuration
#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_repo_path")]
    pub repo_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Config> {
        let path = default_config_path();
        if !path.exists() {
            let Some(parent) = path.parent() else {
                bail!("Could not get parent of path");
            };
            fs::create_dir_all(parent)?;
            fs::write(&path, "")?;
        }
        Ok(toml::from_str(&fs::read_to_string(&path)?)?)
    }
}
