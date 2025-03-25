pub mod asset;
pub mod tar;
pub mod zstd;

use serde::{Deserialize, Serialize};
use tar::TarPack;
use zstd::Zstd;
use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub const MANIFEST_FILE: &'static str = "manifest.toml";

pub type TarZstPack = Zstd<TarPack>;

#[derive(Debug, Error)]
pub enum PackLoadError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("No pack \"{0}\" found")]
    NoSuchPack(&'static str),
}

#[derive(Debug, Error)]
pub enum AssetsLoadError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Deserializer error: {0}")]
    TomlDe(#[from] toml::de::Error),
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(transparent)]
pub struct PackMeta(pub PathBuf);

#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    packs_dir: PathBuf,
    packs: HashMap<String, PackMeta>,
}

pub struct Raw<R: io::Read> {
    pub size_hint: Option<usize>,
    pub path: PathBuf,
    pub read: R,
}

impl Assets {
    pub fn new(packs_dir: impl Into<PathBuf>, packs: HashMap<String, PackMeta>) -> Self {
        Self {
            packs_dir: packs_dir.into(),
            packs,
        }
    }

    pub fn load_from_dir(path: impl AsRef<Path>) -> Result<Self, AssetsLoadError> {
        let path = path.as_ref().canonicalize()?;
        let buf = fs::read_to_string(path.join(MANIFEST_FILE))?;

        let mut assets = toml::from_str::<Assets>(&buf)?;
        assets.packs_dir = path.join(assets.packs_dir);

        Ok(assets)
    }

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
    /// Constructs the asset from pack data
    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> io::Result<Self>;
}

/// Trait defining package loading and asset access operations
pub trait Pack: Sized {
    /// Loads package data from a reader
    fn load(read: impl io::Read) -> io::Result<Self>;

    /// Retrieves [Raw] data stream for an asset
    fn get_raw(&mut self, path: impl AsRef<Path>) -> io::Result<Raw<impl io::Read>>;

    /// Retrieves and constructs a typed asset
    fn get<A: Asset>(&mut self, path: impl AsRef<Path>) -> io::Result<A> {
        A::load(self, path)
    }
}
