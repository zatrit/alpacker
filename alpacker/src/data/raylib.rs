use raylib::{
    audio::{RaylibAudio, Wave},
    texture::Image,
};
use std::{ffi::OsStr, io, path::Path};

use crate::{Asset, AssetResult, Pack};

/// Returns the file extension of the given path as a string, prefixed with a dot.
/// If the path has no extension, the provided default extension is used instead.
fn file_type(path: &Path, default: &str) -> String {
    let mut extension = String::with_capacity(4);
    extension.push('.');
    extension.push_str(path.extension().and_then(OsStr::to_str).unwrap_or(default));
    extension
}

/// Error type for raylib asset operations, supporting both raylib and I/O errors.
#[derive(Debug, thiserror::Error)]
pub enum RaylibError {
    #[error("Raylib error: {0}")]
    Raylib(#[from] raylib::error::Error),

    #[error("I/0 error: {0}")]
    Io(#[from] io::Error),
}

impl Asset for Image {
    type Error = RaylibError;

    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> AssetResult<Self> {
        let path = path.as_ref();
        let data = pack.get::<Vec<u8>>(path)?;
        let ext = file_type(path, "png");

        Image::load_image_from_mem(&ext, &data).map_err(RaylibError::Raylib)
    }
}

/// A type alias for a sprite that uses `raylib::texture::Image` as its image representation.
///
/// This alias is available only when the `aseprite` feature is enabled.
/// It represents a sprite with metadata loaded from an Aseprite file
/// and an associated image stored as a `raylib::texture::Image`,
/// which is compatible with raylib's rendering system.
///
/// See [`Sprite`](super::aseprite::Sprite) for more details.
#[cfg(feature = "aseprite")]
pub type RaylibSprite = super::aseprite::Sprite<Image>;

/// A trait for loading raylib-specific assets, requiring a system dependency (`RaylibAudio`, etc.).
pub trait RaylibAsset<'r>: Sized {
    /// The raylib system type required for asset loading (e.g., `RaylibAudio` for `Wave`).
    type System;

    /// Loads an asset from a pack using the provided raylib system.
    fn load(
        pack: &mut impl Pack,
        system: &'r mut Self::System,
        path: impl AsRef<Path>,
    ) -> Result<Self, RaylibError>;
}

/// Extension trait for `Pack` to provide convenient loading of raylib assets.
pub trait PackRaylibExt<'r>: Pack {
    /// Retrieves a raylib asset from the pack, using the necessary raylib system.
    fn get<A: RaylibAsset<'r>>(
        &mut self,
        raylib: &'r mut A::System,
        path: impl AsRef<Path>,
    ) -> Result<A, RaylibError> {
        A::load(self, raylib, path)
    }
}

/// Implements `PackRaylibExt` for all `Pack` implementations.
impl<P: Pack> PackRaylibExt<'_> for P {}

/// Implements `RaylibAsset` for `Wave`, allowing it to be loaded from a pack.
///
/// - Uses `RaylibAudio` to create a `Wave` from raw audio data.
/// - Determines the file type from the path (default: `.wav`).
/// - Loads the `Wave` into memory using raylib.
impl<'r> RaylibAsset<'r> for Wave<'r> {
    type System = RaylibAudio;

    fn load(
        pack: &mut impl Pack,
        system: &'r mut Self::System,
        path: impl AsRef<Path>,
    ) -> Result<Self, RaylibError> {
        let data = pack.get::<Vec<u8>>(&path)?;
        let ext = file_type(path.as_ref(), "wav");

        system
            .new_wave_from_memory(&ext, &data)
            .map_err(RaylibError::Raylib)
    }
}
