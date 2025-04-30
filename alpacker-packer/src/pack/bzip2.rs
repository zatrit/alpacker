use alpacker::pack::bzip2::Bzip2Codec;
use bzip2::Compression;
use std::{borrow::Cow, io};

use super::codec::Encode;

impl Encode for Bzip2Codec {
    fn encode(write: impl io::Write) -> io::Result<impl io::Write> {
        Ok(bzip2::write::BzEncoder::new(write, Compression::default()))
    }

    fn extension() -> Cow<'static, str> {
        Cow::Borrowed(".bz2")
    }
}
