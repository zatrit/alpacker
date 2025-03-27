use std::{
    collections::HashMap,
    hash::BuildHasher,
    io::{self, Read, Seek},
    path::{Path, PathBuf},
};

use crate::{DefaultHasher, Pack, Raw};

/// TAR archive implementation of the Pack trait
pub struct TarPack<S = DefaultHasher> {
    file_contents: HashMap<PathBuf, Vec<u8>, S>,
    skipped: Vec<Skipped>,
}

#[derive(Debug)]
pub enum Skipped {
    NotAFile(PathBuf),
    Error(io::Error),
}

impl TarPack {
    pub const fn skipped(&self) -> &Vec<Skipped> {
        &self.skipped
    }
}

impl<S: BuildHasher + Default> Pack for TarPack<S> {
    fn get_raw(&mut self, path: impl AsRef<Path>) -> io::Result<Raw<impl Read + Seek>> {
        let path = path.as_ref();

        match self.file_contents.get(path) {
            Some(raw) => Ok(Raw {
                path: path.to_path_buf(),
                size_hint: Some(raw.len()),
                read: io::Cursor::new(raw),
            }),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("No such file: {path:?}"),
            )),
        }
    }

    fn load(read: impl Read) -> io::Result<Self> {
        let mut tar = tar::Archive::new(read);

        let mut file_contents = HashMap::with_hasher(S::default());
        #[allow(unused_mut)]
        let mut skipped = Vec::new();

        for entry in tar.entries()? {
            let entry = match entry {
                Ok(entry) => entry,
                #[allow(unused)]
                Err(err) => {
                    #[cfg(feature = "collect-errors")]
                    skipped.push(Skipped::Error(err));
                    continue;
                }
            };

            let header = entry.header();
            let path = header.path()?.to_path_buf();
            if !header.entry_type().is_file() {
                #[cfg(feature = "collect-errors")]
                skipped.push(Skipped::NotAFile(path));
                continue;
            }

            let size = entry.size();
            let mut buf = Vec::with_capacity(size as usize);
            entry.take(size).read_to_end(&mut buf)?;

            file_contents.insert(path, buf);
        }

        Ok(Self {
            file_contents,
            skipped,
        })
    }
}
