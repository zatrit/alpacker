use std::io::{self, Read};

use lz4_flex::frame::FrameDecoder;

use super::codec::{Decode, EncodedPack};
use crate::Pack;

/// A codec implementation for decoding LZ4-compressed input streams.
/// Used with [`EncodedPack`] to support `.lz4` asset packs.
#[non_exhaustive]
pub struct Lz4Codec;

#[allow(type_alias_bounds)]
pub type Lz4<P: Pack> = EncodedPack<P, Lz4Codec>;

impl Decode for Lz4Codec {
    fn decode(read: impl Read) -> io::Result<impl Read> {
        Ok(FrameDecoder::new(read))
    }
}
