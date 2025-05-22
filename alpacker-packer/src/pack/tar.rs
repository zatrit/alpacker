use alpacker::pack::TarPack;
use std::{borrow::Cow, io, path::Path};

use crate::MakePack;

/// Implements the [MakePack] trait for [TarPack],
/// allowing it to create a `.tar` archive from a directory.
impl MakePack for TarPack {
    /// Creates a TAR archive from the specified directory and writes it to `write`.
    ///
    /// # Arguments
    /// * `root` - The directory to package.
    /// * `write` - The output writer to store the `.tar` file.
    ///
    /// # Returns
    /// * `Ok(())` if the packaging was successful.
    /// * `Err(io::Error)` if an error occurs while reading files or writing the archive.
    fn make(root: impl AsRef<Path>, write: impl io::Write) -> io::Result<()> {
        let root = root.as_ref();
        let mut tar = tar::Builder::new(write);
        tar.append_dir_all("", root)?;
        tar.finish()
    }

    fn extension() -> Cow<'static, str> {
        Cow::Borrowed(".tar")
    }
}
