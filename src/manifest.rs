use anyhow::Result;
use semver::Version;
use serde::{Deserialize, Serialize};

/// Repo-specific configuration and information
#[derive(Serialize, Deserialize)]
pub struct Manifest {
    /// The format version of this repo - generally matches the version of notes that created the
    /// repo
    version: Version,
}

impl Manifest {
    /// Construct a fresh manifest.
    pub fn new() -> Result<Manifest> {
        Ok(Self {
            version: Version::parse(env!("CARGO_PKG_VERSION"))?,
        })
    }
}
