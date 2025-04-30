macro_rules! pack_fmt {
    ($ft: literal, $mod: ident, $use:ident) => {
        #[cfg(feature = $ft)]
        pub mod $mod;
        #[cfg(feature = $ft)]
        pub use ::alpacker::pack::$use;
    };
}

pack_fmt!("tar", tar, TarPack);
pack_fmt!("zstd", zstd, Zstd);
pack_fmt!("bzip2", bzip2, Bzip2);

pub mod codec;
