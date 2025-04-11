use std::{
    io::{self, Read, Seek},
    ops::Deref,
    path::{Path, PathBuf},
};

use crate::{Pack, Raw};

/// A wrapper around a [Pack] implementation that applies [zstd] compression.
/// This allows for transparent decompression of archives compressed with [zstd].
#[derive(Debug)]
pub struct Zstd<P: Pack>(P);

impl<P: Pack> Pack for Zstd<P> {
    fn load(read: impl Read) -> io::Result<Self> {
        let zstd = zstd::Decoder::new(read)?; // Create a Zstd decoder to decompress the input stream.
        P::load(zstd).map(Self) // Load the decompressed archive using the underlying `Pack` implementation.
    }

    #[inline]
    fn get_raw<'p>(&mut self, path: &'p Path) -> io::Result<Raw<'p, impl Read + Seek>> {
        self.0.get_raw(path) // Delegate the call to the inner `Pack` implementation.
    }

    #[inline]
    fn exists(&self, path: impl AsRef<PathBuf>) -> bool {
        self.0.exists(path)
    }
}

impl<P: Pack> Deref for Zstd<P> {
    type Target = P;

    /// Provides immutable access to the underlying `Pack` implementation.
    ///
    /// This allows `Zstd<P>` to behave like `P`, enabling access to its methods
    /// without requiring explicit dereferencing.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
