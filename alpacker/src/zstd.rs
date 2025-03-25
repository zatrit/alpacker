use std::{io, ops::Deref};

use crate::{Pack, Raw};

pub struct Zstd<P: Pack>(P);

impl<P: Pack> Pack for Zstd<P> {
    fn load(read: impl io::Read) -> io::Result<Self> {
        let zstd = zstd::Decoder::new(read)?;
        P::load(zstd).map(Self)
    }

    fn get_raw(&mut self, path: impl AsRef<std::path::Path>) -> io::Result<Raw<impl io::Read>> {
        self.0.get_raw(path)
    }
}

impl<P: Pack> Deref for Zstd<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
