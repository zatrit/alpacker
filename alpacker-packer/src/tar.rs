use std::{
    fs::{File, read_dir},
    io,
    path::Path,
};

use alpacker::pack::TarPack;

use crate::MakePack;

impl MakePack for TarPack {
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

    fn suffix() -> String {
        String::from(".tar")
    }
}
