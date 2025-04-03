use image::{DynamicImage, ImageError, ImageFormat};
use std::{io::BufReader, path::Path};

use crate::{Asset, AssetResult, Pack};

/// Implementation of the Asset trait for DynamicImage from the image crate
///
/// Enables loading images through the asset pipeline using the [Pack] abstraction
impl Asset for DynamicImage {
    type Error = ImageError; // Uses image crate's error type for decoding failures

    /// Loads an image from the asset pack
    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> AssetResult<Self> {
        let raw = pack.get_raw(&path)?; // Propagates pack's error

        let buf_read = BufReader::new(raw.read);
        let format = ImageFormat::from_path(&path)?;

        image::load(buf_read, format)
    }
}

/// A type alias for a sprite that uses `DynamicImage` as its image representation.
///
/// This alias is available only when the `aseprite` feature is enabled.
/// It represents a sprite with metadata loaded from an Aseprite file
/// and an associated image stored as a `DynamicImage`.
///
/// See [`Sprite`](super::aseprite::Sprite) for more details.
#[cfg(feature = "aseprite")]
pub type ImageSprite = super::aseprite::Sprite<DynamicImage>;
