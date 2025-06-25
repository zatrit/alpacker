pub mod pack;
pub mod transform;

#[allow(unused)]
pub use alpacker::pack::*;
use alpacker::{Assets, JsonIoError, MANIFEST_FILE, Pack, PackManifest, PackMeta};
use std::{
    borrow::Cow,
    collections::HashMap,
    env,
    fs::{self, File, create_dir},
    io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

/// Trait for creating a package from a directory.
/// Implementing types must define how the package is created (`make`)
/// and provide a file suffix (`suffix`).
pub trait MakePack {
    /// Creates a package from the specified directory and writes it to `write`.
    fn make(
        root: impl AsRef<Path>,
        write: impl io::Write,
        manifest: PackManifest,
    ) -> io::Result<()>;

    /// Returns the file extension for the package type.
    fn extension() -> Cow<'static, str>;
}

/// Trait for applying transformations to files in a directory.
pub trait Transform {
    type Error;

    /// Transforms the files in the given path.
    fn transform(&mut self, path: impl AsRef<Path>) -> Result<(), Self::Error>;
}

/// A utility for building package directories before creating a pack.
pub struct PackBuilder {
    /// Temporary directory for building the package
    work_dir: PathBuf,

    /// Whether to remove the working directory on drop
    cleanup_on_drop: bool,
}

// https://stackoverflow.com/a/65192210
/// Recursively copies all files and directories from `src` to `dst`.
/// Ensures that `dst` is a directory before proceeding.
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let dst = dst.as_ref();

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
    /// Creates a new `PackBuilder` with a unique temporary directory.
    pub fn new() -> io::Result<PackBuilder> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp = env::temp_dir().join(format!("pack-tmp-{time}"));
        fs::create_dir(&temp)?;
        Ok(Self::with_temp_dir(temp, false))
    }

    /// Creates a `PackBuilder` with a specified temporary directory.
    /// The `remove_dir` flag determines whether the directory should be deleted on drop.
    pub fn with_temp_dir(dir: impl Into<PathBuf>, cleanup_on_drop: bool) -> Self {
        Self {
            work_dir: dir.into(),
            cleanup_on_drop,
        }
    }

    #[inline]
    pub fn transform<T: Transform>(self, transformer: &mut T) -> Result<Self, T::Error> {
        transformer.transform(&self.work_dir)?;
        Ok(self)
    }

    /// Copies files from the source directory into the package directory.
    #[inline]
    pub fn copy_from(self, src: impl AsRef<Path>) -> io::Result<Self> {
        copy_dir_all(src, &self.work_dir)?;
        Ok(self)
    }

    #[inline]
    pub fn write_pack<P: MakePack>(&self, write: impl io::Write) -> io::Result<()> {
        let manifest = PackManifest {
            entry_count: WalkDir::new(&self.work_dir).into_iter().flatten().count(),
            file_count: WalkDir::new(&self.work_dir)
                .into_iter()
                .flatten()
                .filter_map(|entry| entry.metadata().ok())
                .filter(|meta| meta.is_file())
                .count(),
        };

        P::make(&self.work_dir, write, manifest)
    }

    pub fn insert_file(&mut self, path: impl AsRef<Path>, content: &[u8]) -> io::Result<()> {
        let file_path = self.work_dir.join(path);
        fs::write(file_path, content)?;
        Ok(())
    }

    pub const fn work_dir(&self) -> &PathBuf {
        &self.work_dir
    }
}

impl Drop for PackBuilder {
    /// Removes the temporary directory when the `PackBuilder` is dropped,
    /// if the `remove_dir` flag is set to `true`.
    fn drop(&mut self) {
        if self.cleanup_on_drop {
            if let Err(err) = fs::remove_dir_all(&self.work_dir) {
                eprintln!("Warning: Failed to remove temp dir: {err}");
            }
        }
    }
}

/// Builder for managing assets and packaging them into a manifest.
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

    /// Adds a package to the builder, creates the package file, and updates the manifest.
    pub fn add_pack<P: MakePack>(mut self, name: &str, pack: &PackBuilder) -> io::Result<Self> {
        let mut file_name = name.to_string();
        file_name.push_str(&P::extension());
        let path = self.root.join(&self.packs_dir).join(&file_name);

        let mut file = File::create_new(&path)?;
        pack.write_pack::<P>(&mut file)?;

        let meta = PackMeta(PathBuf::from(file_name));
        self.packs.insert(name.to_string(), meta);

        Ok(self)
    }

    /// Writes the asset manifest (`manifest.json`) containing metadata about packaged assets.
    pub fn write_manifest(self, allow_overwrite: bool) -> Result<(), JsonIoError> {
        let manifest_path = self.root.join(MANIFEST_FILE);

        let assets = Assets::new(self.packs_dir, self.packs);

        let file = match allow_overwrite {
            true => File::create(manifest_path),
            false => File::create_new(manifest_path),
        }?;
        serde_json::to_writer(file, &assets)?;

        Ok(())
    }
}
