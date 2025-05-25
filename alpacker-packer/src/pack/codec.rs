use std::{
    borrow::Cow,
    io::{self, Write},
    path::Path,
};

use alpacker::{
    PackManifest,
    pack::codec::{Decode, EncodedPack},
};

use crate::MakePack;

/// A trait for types that can encode (compress) output streams for packaging.
///
/// This is the inverse of [`Decode`] and is used by pack builders
/// to write compressed archive formats like `.tar.zst` or `.tar.bz2`.
pub trait Encode {
    /// Wraps the given writer in an encoder.
    fn encode(write: impl Write) -> io::Result<impl Write>;

    /// Returns the file extension (e.g., ".zst", ".bz2") for this encoder.
    fn extension() -> Cow<'static, str>;
}

impl<P: MakePack, C: Encode + Decode> MakePack for EncodedPack<P, C> {
    fn make(root: impl AsRef<Path>, write: impl Write, manifest: PackManifest) -> io::Result<()> {
        P::make(root, C::encode(write)?, manifest)
    }

    fn extension() -> Cow<'static, str> {
        let mut ext = P::extension().to_string();
        ext.push_str(C::extension().as_ref());
        Cow::Owned(ext)
    }
}
