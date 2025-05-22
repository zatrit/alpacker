use alpacker::pack::lz4::Lz4Codec;
use std::{borrow::Cow, io};

use super::codec::Encode;

impl Encode for Lz4Codec {
    fn encode(write: impl io::Write) -> io::Result<impl io::Write> {
        Ok(lz4_flex::frame::FrameEncoder::new(write))
    }

    fn extension() -> Cow<'static, str> {
        Cow::Borrowed(".lz4")
    }
}
