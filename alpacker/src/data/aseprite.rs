/* Disclaimer:
The term "Aseprite" is used solely to refer to the file format exported by the Aseprite software.
This project is not an official product, nor is it affiliated with or endorsed by the developers of Aseprite.
All trademarks and logos, including "Aseprite", are the property of their respective owners. */

/* This module provides functionality for loading sprite assets that combine metadata (like Aseprite spritesheets)
and their associated image data. It handles path resolution, error propagation, and asset loading in a generic way. */

use aseprite::SpritesheetData;
use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::{Asset, AssetResult, JsonIoError, Pack};

// Implementation of Asset trait for Aseprite SpritesheetData
impl Asset for SpritesheetData {
    type Error = JsonIoError;

    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> AssetResult<Self> {
        let raw = pack.get_raw(path)?;
        serde_json::from_reader(raw.read).map_err(JsonIoError::Json)
    }
}

/// Trait for assets that can provide associated image paths
pub trait SpriteMeta: Asset {
    fn image_path(&self, meta_path: impl AsRef<Path>) -> Option<PathBuf>;
}

impl SpriteMeta for SpritesheetData {
    fn image_path(&self, meta_path: impl AsRef<Path>) -> Option<PathBuf> {
        let empty = PathBuf::new();

        match &self.meta.image {
            Some(image) => Some(meta_path.as_ref().parent().unwrap_or(&empty).join(image)),
            None => None,
        }
    }
}

/// Container combining loaded metadata and its associated image
pub struct Sprite<I: Asset, M: Asset = SpritesheetData>
where
    I::Error: Error,
    M::Error: Error,
{
    pub image: Option<I>,
    pub meta: M,
}

/// Unified error type for sprite loading operations
#[derive(Debug, Error)]
pub enum SpriteError<M: Error, I: Error> {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Metadata error: {0}")]
    Meta(M),

    #[error("Image error: {0}")]
    Image(I),
}

impl<I: Asset, M: SpriteMeta> Asset for Sprite<I, M>
where
    I::Error: Error,
    M::Error: Error,
{
    type Error = SpriteError<M::Error, I::Error>;

    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> AssetResult<Self> {
        let meta = M::load(pack, &path).map_err(SpriteError::Meta)?;
        let image = match meta.image_path(path) {
            Some(path) => Some(I::load(pack, path).map_err(SpriteError::Image)?),
            None => None,
        };

        Ok(Self { image, meta })
    }
}
