use std::{
    io,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::{Pack, Raw};

/// A trait for types that can decode compressed input streams (e.g. decompressors).
///
/// Used by [`EncodedPack`] to implement runtime decompression before loading a [`Pack`].
pub trait Decode {
    /// Returns a decoder over the provided input stream.
    fn decode(read: impl io::Read) -> io::Result<impl io::Read>;
}

/// A generic wrapper around a [`Pack`] that transparently decodes its input stream
/// using the specified codec `C`, which implements [`Decode`].
///
/// This enables composing any pack type with a compression format like Zstd or Bzip2.
///
/// # Type Parameters
/// - `P`: The base pack implementation (e.g. `TarPack`)
/// - `C`: A codec that implements [`Decode`]
pub struct EncodedPack<P, C> {
    pub pack: P,
    _d: PhantomData<C>,
}

impl<P: Pack, C: Decode> Pack for EncodedPack<P, C> {
    fn load(read: impl io::Read) -> io::Result<Self> {
        Ok(Self {
            pack: P::load(C::decode(read)?)?,
            _d: PhantomData,
        })
    }

    #[inline(always)]
    fn get_raw<'p>(&mut self, path: &'p Path) -> io::Result<Raw<'p, impl io::Read + io::Seek>> {
        self.pack.get_raw(path)
    }

    #[inline(always)]
    fn exists(&self, path: impl AsRef<Path>) -> bool {
        self.pack.exists(path)
    }
}

impl<P: Pack, C> Deref for EncodedPack<P, C> {
    type Target = P;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl<P: Pack, C> DerefMut for EncodedPack<P, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}
