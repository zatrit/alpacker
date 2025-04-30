use bzip2::read::BzDecoder;
use std::io::{self, Read};

use super::codec::{Decode, EncodedPack};
use crate::Pack;

/// A codec implementation for decoding Bzip2-compressed input streams.
/// Used with [`EncodedPack`] to support `.bz2` asset packs.
#[non_exhaustive]
pub struct Bzip2Codec;

#[allow(type_alias_bounds)]
pub type Bzip2<P: Pack> = EncodedPack<P, Bzip2Codec>;

impl Decode for Bzip2Codec {
    fn decode(read: impl Read) -> io::Result<impl Read> {
        Ok(BzDecoder::new(read))
    }
}
