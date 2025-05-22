pub mod codec;

macro_rules! pack_type {
    ($ft: literal, $mod: ident, $use:ident) => {
        #[cfg(feature = $ft)]
        pub mod $mod;
        #[cfg(feature = $ft)]
        pub use $mod::$use;
    };
}

macro_rules! pack_alchemy {
    ($($($feature:literal),* => $alias:ident = $type:ty [$doc:literal];)*) => {
        $(#[doc = $doc]
        #[cfg(all($(feature = $feature),*))]
        pub type $alias = $type;)*
    };
}

pack_type!("tar", tar, TarPack);
pack_type!("zstd", zstd, Zstd);
pack_type!("bzip2", bzip2, Bzip2);
pack_type!("lz4", lz4, Lz4);

pack_alchemy!(
    "tar", "zstd" => TarZstPack = Zstd<TarPack> ["Zstandard compressed TAR pack"];
    "tar", "bzip2" => TarBz2Pack = Bzip2<TarPack> ["Bzip2 compressed TAR pack"];
    "tar", "lz4" => TarLz4Pack = Lz4<TarPack> ["LZ4 compressed TAR pack"];
);
