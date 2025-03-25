use std::{
    io::{self, Read},
    path::Path,
};

use crate::{Asset, Pack};

impl Asset for String {
    fn load(pack: &mut impl Pack, path: impl AsRef<Path>) -> io::Result<Self> {
        let mut raw = pack.get_raw(path)?;

        let size = raw.size_hint.unwrap_or(0);
        let mut buf = String::with_capacity(size);
        raw.read.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
