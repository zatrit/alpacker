use alpacker::zstd::Zstd;

use crate::MakePack;

impl<P: MakePack> MakePack for Zstd<P> {
    fn make(root: impl AsRef<std::path::Path>, write: impl std::io::Write) -> std::io::Result<()> {
        let zstd = zstd::Encoder::new(write, 7)?.auto_finish();
        P::make(root, zstd)
    }

    fn suffix() -> String {
        format!("{}.zst", P::suffix())
    }
}
