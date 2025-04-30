use std::{
    collections::HashMap,
    hash::BuildHasher,
    io::{self, Read, Seek},
    path::{Path, PathBuf},
};

use crate::{DefaultHasher, Pack, Raw};

/// TAR archive implementation of the Pack trait
#[derive(Debug)]
pub struct TarPack<S = DefaultHasher> {
    /// Stores the contents of the files in the archive as a hash map,
    /// where the key is the file path and the value is the file content.
    contents: HashMap<PathBuf, Vec<u8>, S>,

    /// Keeps track of files that were skipped during extraction.
    skipped: Vec<Skipped>,
}

/// Enum representing the reasons why a file was skipped.
#[derive(Debug)]
pub enum Skipped {
    /// The path does not point to a valid file (e.g., it could be a directory).
    NotAFile(PathBuf),
}

impl TarPack {
    /// Returns a reference to the list of skipped files.
    pub const fn skipped(&self) -> &Vec<Skipped> {
        &self.skipped
    }
}

impl<S: BuildHasher + Default> Pack for TarPack<S> {
    fn get_raw<'p>(&mut self, path: &'p Path) -> io::Result<Raw<'p, impl Read + Seek>> {
        match self.contents.get(path) {
            Some(raw) => Ok(Raw {
                path,
                size_hint: Some(raw.len()), // Provide an estimated file size
                read: io::Cursor::new(raw), // Wrap the file contents in an in-memory reader
            }),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("No such file: {path:?}"),
            )),
        }
    }

    fn load(read: impl Read) -> io::Result<Self> {
        let mut tar = tar::Archive::new(read);

        // Create a hash map for storing file contents with the specified hasher.
        let mut contents = HashMap::with_hasher(S::default());
        // List of skipped files (only used if the "collect-errors" feature is enabled).
        #[allow(unused_mut)]
        let mut skipped = Vec::new();

        // Iterate over each entry in the TAR archive.
        for entry in tar.entries()? {
            let entry = entry?;

            let header = entry.header();
            let path = header.path()?.to_path_buf();
            if !header.entry_type().is_file() {
                #[cfg(feature = "collect-errors")]
                skipped.push(Skipped::NotAFile(path));
                continue;
            }

            // Read the file contents into a buffer.
            let size = entry.size();
            let mut buf = Vec::with_capacity(size as usize);
            entry.take(size).read_to_end(&mut buf)?;

            contents.insert(path, buf);
        }

        Ok(Self { contents, skipped })
    }

    fn exists(&self, path: impl AsRef<Path>) -> bool {
        self.contents.contains_key(path.as_ref())
    }
}
