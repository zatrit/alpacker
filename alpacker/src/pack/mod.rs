#[cfg(feature = "tar")]
mod tar;
#[cfg(feature = "zstd")]
mod zstd;

#[cfg(feature = "tar")]
pub use tar::TarPack;
#[cfg(feature = "zstd")]
pub use zstd::Zstd;

#[cfg(all(feature = "tar", feature = "zstd"))]
pub type TarZstPack = Zstd<TarPack>;
