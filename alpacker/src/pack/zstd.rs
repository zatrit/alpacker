use std::io::{self, Read};

use super::codec::{Decode, EncodedPack};
use crate::Pack;

/// A codec implementation for decoding Zstandard-compressed input streams.
/// Used with [`EncodedPack`] to transparently decompress `.zst` files.
#[non_exhaustive]
pub struct ZstdCodec;

#[allow(type_alias_bounds)]
pub type Zstd<P: Pack> = EncodedPack<P, ZstdCodec>;

impl Decode for ZstdCodec {
    fn decode(read: impl Read) -> io::Result<impl Read> {
        zstd::Decoder::new(read)
    }
}
