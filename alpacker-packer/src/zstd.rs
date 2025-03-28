use std::{borrow::Cow, io, path::Path};

use alpacker::pack::Zstd;

use crate::MakePack;

/// Implements [MakePack] for `Zstd<P>`, allowing compression of any [MakePack] type using Zstandard.
impl<P: MakePack> MakePack for Zstd<P> {
    /// Compresses the output of `P::make` using Zstandard compression.
    ///
    /// # Arguments
    /// * `root` - The directory to package.
    /// * `write` - The output writer where the compressed archive will be stored.
    ///
    /// # Returns
    /// * `Ok(())` if compression and packaging succeed.
    /// * `Err(io::Error)` if an error occurs during processing.
    fn make(root: impl AsRef<Path>, write: impl io::Write) -> io::Result<()> {
        let zstd = zstd::Encoder::new(write, 7)?.auto_finish();
        P::make(root, zstd)
    }

    fn suffix() -> Cow<'static, str> {
        Cow::Owned(format!("{}.zst", P::suffix()))
    }
}
