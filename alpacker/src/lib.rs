pub mod data;
pub mod pack;

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    hash, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

/// The name of the manifest file that stores metadata about available asset packs.
pub const MANIFEST_FILE: &'static str = "manifest.json";

// Defines the default hasher to use for hash maps.
// Uses `twox-hash` if the feature is enabled, otherwise falls back to `RandomState`.
#[cfg(feature = "twox-hash")]
pub type DefaultHasher = hash::BuildHasherDefault<twox_hash::XxHash64>;

#[cfg(not(feature = "twox-hash"))]
pub type DefaultHasher = hash::RandomState;

/// A type alias for [Asset] loading results, associating each asset with its error type.
#[allow(type_alias_bounds)]
pub type AssetResult<A: Asset> = Result<A, A::Error>;

#[derive(Debug, Error)]
pub enum PackLoadError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("No pack \"{0}\" found")]
    NoSuchPack(&'static str),
}

#[derive(Debug, Error)]
pub enum JsonIoError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Deserializer error: {0}")]
    Json(#[from] serde_json::Error),
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

/// Represents a raw file inside an asset pack.
pub struct Raw<R: io::Read + io::Seek> {
    pub size_hint: Option<usize>,
    pub path: PathBuf,
    pub read: R,
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

        let mut assets = serde_json::from_str::<Assets>(&buf)?;
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
    pub fn load_pack<P: Pack>(&self, name: &'static str) -> Result<P, PackLoadError> {
        let Some(meta) = self.packs.get(name) else {
            return Err(PackLoadError::NoSuchPack(name));
        };

        let path = self.packs_dir.join(&meta.0).canonicalize()?;
        let file = File::open(path)?;

        P::load(file).map_err(PackLoadError::Io)
    }
}

/// Trait for types that can be loaded from asset packs
pub trait Asset: Sized {
    type Error;

    /// Constructs an asset from a given pack.
    ///
    /// # Arguments
    /// * `pack` - The pack to load the asset from.
    /// * `path` - The path to the asset inside the pack.
    ///
    /// # Returns
    /// * `Ok(A)` if the asset is successfully loaded.
    /// * `Err(Self::Error)` if an error occurs.
    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> AssetResult<Self>;
}

/// Trait defining package loading and asset access operations
pub trait Pack: Sized {
    /// Loads package data from a readable stream.
    ///
    /// # Arguments
    /// * `read` - The input stream containing the package data.
    ///
    /// # Returns
    /// * `Ok(Self)` if the package is successfully loaded.
    /// * `Err(io::Error)` if an error occurs.
    fn load(read: impl io::Read) -> io::Result<Self>;

    /// Retrieves a [Raw] object representing a file inside the archive.
    ///
    /// # Arguments
    /// * `path` - The path of the file to retrieve.
    ///
    /// # Returns
    /// * `Ok(Raw<impl Read + Seek>)` if the file is found.
    /// * `Err(io::Error)` if the file is missing.
    fn get_raw(&mut self, path: impl AsRef<Path>) -> io::Result<Raw<impl io::Read + io::Seek>>;

    /// Retrieves and constructs a typed asset.
    ///
    /// This is a convenience method that calls `A::load(self, path)`.
    ///
    /// # Arguments
    /// * `path` - The path of the asset to retrieve.
    ///
    /// # Returns
    /// * `Ok(A)` if the asset is successfully loaded.
    /// * `Err(A::Error)` if an error occurs.
    fn get<A: Asset>(&mut self, path: impl AsRef<Path>) -> Result<A, A::Error> {
        A::load(self, path)
    }

    fn exists(&self, path: impl AsRef<PathBuf>) -> bool;
}
