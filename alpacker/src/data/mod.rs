#[cfg(feature = "aseprite")]
pub mod aseprite;

#[cfg(feature = "image")]
pub mod image;

use std::{
    io::{Error, Read},
    path::Path,
};

use crate::{Asset, AssetResult, Pack};

impl Asset for String {
    type Error = Error;

    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> AssetResult<Self> {
        let mut raw = pack.get_raw(path)?;

        let size = raw.size_hint.unwrap_or(0);
        let mut buf = String::with_capacity(size);
        raw.read.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
