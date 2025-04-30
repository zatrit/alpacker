pub mod codec;

macro_rules! pack_fmt {
    ($ft: literal, $mod: ident, $use:ident) => {
        #[cfg(feature = $ft)]
        pub mod $mod;
        #[cfg(feature = $ft)]
        pub use $mod::$use;
    };
}

pack_fmt!("tar", tar, TarPack);
pack_fmt!("zstd", zstd, Zstd);
pack_fmt!("bzip2", bzip2, Bzip2);

/// Type alias for a TAR archive compressed with Zstandard.
///
/// Equivalent to: `EncodedPack<TarPack, ZstdCodec>`
#[cfg(all(feature = "tar", feature = "zstd"))]
pub type TarZstPack = Zstd<TarPack>;

/// Type alias for a TAR archive compressed with Bzip2.
///
/// Equivalent to: `EncodedPack<TarPack, Bzip2Codec>`
#[cfg(all(feature = "tar", feature = "bzip2"))]
pub type TarBz2Pack = Bzip2<TarPack>;
