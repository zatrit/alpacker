use alpacker::pack::zstd::ZstdCodec;
use std::{borrow::Cow, io};

use super::codec::Encode;

impl Encode for ZstdCodec {
    fn encode(write: impl io::Write) -> io::Result<impl io::Write> {
        Ok(zstd::Encoder::new(write, 7)?.auto_finish())
    }

    fn extension() -> Cow<'static, str> {
        Cow::Borrowed(".zst")
    }
}
