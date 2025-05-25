pub mod data;
pub mod pack;

#[cfg(feature = "fs")]
mod fs;
#[cfg(feature = "fs")]
pub use fs::*;
use serde::{Deserialize, Serialize};

use std::{hash, io, path::Path};
use thiserror::Error;

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
pub enum JsonIoError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Deserializer error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PackManifest {
    pub entry_count: usize,
    pub file_count: usize,
}

/// Represents a raw file inside an asset pack.
pub struct Raw<'p, R: io::Read + io::Seek> {
    pub size_hint: Option<usize>,
    pub path: &'p Path,
    pub read: R,
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
    fn get_raw<'p>(&mut self, path: &'p Path) -> io::Result<Raw<'p, impl io::Read + io::Seek>>;

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

    fn exists(&self, path: impl AsRef<Path>) -> bool;
}
