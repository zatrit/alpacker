use std::{
    borrow::Cow,
    fs::{File, read_dir},
    io,
    path::Path,
};

use alpacker::pack::TarPack;

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

        for entry in read_dir(root)? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("{err}");
                    continue;
                }
            };

            let name = entry.file_name();
            let path = root.join(&name);

            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                tar.append_dir_all(&name, path)?;
            } else if file_type.is_file() {
                let mut file = File::open(path)?;
                tar.append_file(&name, &mut file)?;
            }
        }

        tar.finish()
    }

    fn suffix() -> Cow<'static, str> {
        Cow::Borrowed(".tar")
    }
}
