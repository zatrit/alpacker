use raylib::texture::Image;
use std::{ffi::OsStr, io, path::Path};

use crate::{Asset, AssetResult, Pack};

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

        let mut extension = String::with_capacity(4);
        extension.push('.');
        extension.push_str(path.extension().and_then(OsStr::to_str).unwrap_or("png"));

        Image::load_image_from_mem(&extension, &data).map_err(RaylibError::Raylib)
    }
}
