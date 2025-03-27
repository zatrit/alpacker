use image::{DynamicImage, ImageError, ImageReader};
use std::{io::BufReader, path::Path};

use crate::{Asset, AssetResult, Pack};

/// Implementation of the Asset trait for DynamicImage from the image crate
/// 
/// Enables loading images through the asset pipeline using the [Pack] abstraction
impl Asset for DynamicImage {
    type Error = ImageError;  // Uses image crate's error type for decoding failures

    /// Loads an image from the asset pack
    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> AssetResult<Self> {
        // Get raw byte stream from pack implementation
        let raw = pack.get_raw(path)?;  // Propagates pack's error
        
        // Create buffered reader for efficient byte access
        let buf_read = BufReader::new(raw.read);
        
        // Decode image using image crate's auto-detection:
        // - Determines format from content (PNG, JPEG, etc.)
        // - Handles format-specific parsing
        // - Converts to unified DynamicImage representation
        ImageReader::new(buf_read).decode()
    }
}
