use std::{io, path::Path};

use alpacker::pack::Zstd;

use crate::MakePack;

impl<P: MakePack> MakePack for Zstd<P> {
    fn make(root: impl AsRef<Path>, write: impl io::Write) -> io::Result<()> {
        let zstd = zstd::Encoder::new(write, 7)?.auto_finish();
        P::make(root, zstd)
    }

    fn suffix() -> String {
        format!("{}.zst", P::suffix())
    }
}
