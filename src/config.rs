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

impl Default for Config {
    fn default() -> Self {
        Self {
            repo_path: default_repo_path(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Config> {
        let path = default_config_path();
        if !path.exists() {
            let Some(parent) = path.parent() else {
                bail!("Could not get parent of path");
            };
            fs::create_dir_all(parent)?;
            fs::write(&path, toml::to_string(&Config::default())?)?;
        }
        Ok(toml::from_str(&fs::read_to_string(&path)?)?)
    }
}
