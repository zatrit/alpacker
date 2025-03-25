pub mod tar;
pub mod transform;
pub mod zstd;

use alpacker::{Assets, Pack, PackMeta};
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File, create_dir},
    io::{self, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

pub trait MakePack: Pack {
    fn make(root: impl AsRef<Path>, write: impl Write) -> io::Result<()>;

    fn suffix() -> String;
}

pub trait Transform {
    type Error: Error;

    fn transform(&mut self, path: impl AsRef<Path>) -> Result<(), Self::Error>;
}

pub struct TransformTag {
    pub name: String,
    pub hash: u64,
}

pub struct PackBuilder {
    temp_dir: PathBuf,
    remove_dir: bool,
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("TOML serializer error: {0}")]
    TomlSer(#[from] toml::ser::Error),
}

// https://stackoverflow.com/a/65192210
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let dst = dst.as_ref();
    if !fs::metadata(dst)?.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Not a directory: {:?}", dst),
        ));
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            let dst = dst.join(entry.file_name());
            fs::create_dir_all(&dst)?;
            copy_dir_all(entry.path(), dst)?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }

    Ok(())
}

impl PackBuilder {
    pub fn new(name: &str) -> io::Result<PackBuilder> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp = env::temp_dir().join(format!("{}-pack-{}", name, time));
        fs::create_dir(&temp)?;
        Ok(Self::with_temp_dir(temp, false))
    }

    pub fn with_temp_dir(dir: impl Into<PathBuf>, remove_dir: bool) -> Self {
        Self {
            temp_dir: dir.into(),
            remove_dir,
        }
    }

    #[inline]
    pub fn transform<T: Transform>(self, transformer: &mut T) -> Result<Self, T::Error> {
        transformer.transform(&self.temp_dir)?;
        Ok(self)
    }

    #[inline]
    pub fn copy_from(self, src: impl AsRef<Path>) -> io::Result<Self> {
        copy_dir_all(src, &self.temp_dir)?;
        Ok(self)
    }

    #[inline]
    pub fn make_pack<P: MakePack>(&self, write: impl Write) -> io::Result<()> {
        P::make(&self.temp_dir, write)
    }
}

impl Drop for PackBuilder {
    fn drop(&mut self) {
        if self.remove_dir {
            fs::remove_dir_all(&self.temp_dir).expect("Unable to delete directory");
        }
    }
}

#[derive(Debug)]
pub struct AssetsBuilder {
    root: PathBuf,
    packs_dir: PathBuf,
    packs: HashMap<String, PackMeta>,
}

impl AssetsBuilder {
    pub fn new(root: impl Into<PathBuf>, packs_dir: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        let packs_dir = packs_dir.into();

        match create_dir(root.join(&packs_dir)) {
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {}
            result => result?,
        };

        Ok(Self {
            root,
            packs_dir,
            packs: HashMap::new(),
        })
    }

    pub fn add_pack<P: MakePack>(mut self, name: &str, pack: &PackBuilder) -> io::Result<Self> {
        let mut file_name = name.to_string();
        file_name.push_str(&P::suffix());
        let path = self.root.join(&self.packs_dir).join(&file_name);

        let mut file = File::create_new(&path)?;
        pack.make_pack::<P>(&mut file)?;

        let meta = PackMeta(PathBuf::from(file_name));
        self.packs.insert(name.to_string(), meta);

        Ok(self)
    }

    pub fn write_manifest(self) -> Result<(), ManifestError> {
        let manifest_path = self.root.join("manifest.toml");

        let assets = Assets::new(self.packs_dir, self.packs);

        let toml_string = toml::to_string_pretty(&assets)?;
        fs::write(manifest_path, toml_string)?;

        Ok(())
    }
}
