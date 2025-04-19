use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{JsonIoError, Pack};

/// The name of the manifest file that stores metadata about available asset packs.
pub const MANIFEST_FILE: &str = "manifest.json";

#[derive(Debug, Error)]
pub enum PackLoadError<'a> {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("No pack \"{0}\" found")]
    NoSuchPack(&'a str),
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(transparent)]
pub struct PackMeta(pub PathBuf);

/// Represents a collection of asset packs.
///
/// This struct keeps track of the directory containing asset packs
/// and a mapping of pack names to their metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    packs_dir: PathBuf,
    packs: HashMap<String, PackMeta>,
}

impl Assets {
    /// Creates a new [Assets] instance.
    ///
    /// # Arguments
    /// * `packs_dir` - The root directory where asset packs are stored.
    /// * `packs` - A mapping of pack names to their metadata.
    pub fn new(packs_dir: impl Into<PathBuf>, packs: HashMap<String, PackMeta>) -> Self {
        let packs_dir = packs_dir.into();
        Self { packs_dir, packs }
    }

    /// Loads asset metadata from a directory containing a manifest file.
    ///
    /// # Arguments
    /// * `path` - The directory containing the `manifest.json` file.
    ///
    /// # Returns
    /// [Ok]
    /// * `Ok(Assets)` if the manifest is successfully loaded.
    /// * `Err(JsonIoError)` if an I/O or deserialization error occurs.
    pub fn load_from_dir(path: impl AsRef<Path>) -> Result<Self, JsonIoError> {
        let path = path.as_ref().canonicalize()?;
        let buf = fs::read_to_string(path.join(MANIFEST_FILE))?;

        let mut assets = serde_json::from_str::<Self>(&buf)?;
        assets.packs_dir = path.join(assets.packs_dir);

        Ok(assets)
    }

    /// Loads an asset pack by name.
    ///
    /// # Arguments
    /// * `name` - The name of the asset pack to load.
    ///
    /// # Returns
    /// * `Ok(P)` if the pack is successfully loaded.
    /// * `Err(PackLoadError)` if the pack is missing or fails to load.
    pub fn load_pack<'a, P: Pack>(&self, name: &'a str) -> Result<P, PackLoadError<'a>> {
        let Some(meta) = self.packs.get(name) else {
            return Err(PackLoadError::NoSuchPack(name));
        };

        let path = self.packs_dir.join(&meta.0).canonicalize()?;
        let file = File::open(path)?;

        P::load(file).map_err(PackLoadError::Io)
    }
}
