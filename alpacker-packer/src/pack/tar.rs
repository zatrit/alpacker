use alpacker::{MANIFEST_FILE, PackManifest, pack::TarPack};
use std::{borrow::Cow, io, path::Path};
use tar::Header;

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
    fn make(
        root: impl AsRef<Path>,
        write: impl io::Write,
        manifest: PackManifest,
    ) -> io::Result<()> {
        let root = root.as_ref();

        if root.join(MANIFEST_FILE).exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("`{MANIFEST_FILE}` is reserved for the pack manifest"),
            ));
        }

        let mut tar = tar::Builder::new(write);

        let data = serde_json::to_vec(&manifest)?;
        let mut header = Header::new_gnu();
        header.set_mode(0o644);
        header.set_size(data.len() as u64);
        tar.append_data(&mut header, Path::new(MANIFEST_FILE), io::Cursor::new(data))?;

        tar.append_dir_all("", root)?;
        tar.finish()
    }

    fn extension() -> Cow<'static, str> {
        Cow::Borrowed(".tar")
    }
}
