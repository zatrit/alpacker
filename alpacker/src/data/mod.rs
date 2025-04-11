#[cfg(feature = "aseprite")]
pub mod aseprite;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "raylib")]
pub mod raylib;

use std::{
    io::{Error, Read},
    path::Path,
};

use crate::{Asset, AssetResult, Pack};

impl Asset for String {
    type Error = Error;

    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> AssetResult<Self> {
        let mut raw = pack.get_raw(path.as_ref())?;

        let size = raw.size_hint.unwrap_or(0);
        let mut buf = String::with_capacity(size);
        raw.read.read_to_string(&mut buf)?;
        Ok(buf)
    }
}

impl Asset for Vec<u8> {
    type Error = Error;

    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> AssetResult<Self> {
        let mut raw = pack.get_raw(path.as_ref())?;

        let size = raw.size_hint.unwrap_or(0);
        let mut buf = Vec::with_capacity(size);
        raw.read.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
