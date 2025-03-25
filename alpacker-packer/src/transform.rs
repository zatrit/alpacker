pub use oxipng;

use std::path::Path;

use glob::{GlobError, PatternError, glob};
use oxipng::{OutFile, optimize};
use thiserror::Error;

use crate::Transform;

// oxipng transform
#[derive(Debug, Error)]
pub enum OxipngError {
    #[error("Pattern error: {0}")]
    Pattern(#[from] PatternError),

    #[error("Glob error: {0}")]
    Glob(#[from] GlobError),

    #[error("Oxipng error: {0}")]
    Oxipng(#[from] oxipng::PngError),
}

pub struct OxipngTransform(pub oxipng::Options);

impl Transform for OxipngTransform {
    type Error = OxipngError;

    fn transform(&mut self, path: impl AsRef<Path>) -> Result<(), Self::Error> {
        let path = path.as_ref().join("*.png");

        for path in glob(path.to_str().unwrap())? {
            let path = path?;
            optimize(
                &oxipng::InFile::Path(path.clone()),
                &OutFile::Path {
                    path: Some(path),
                    preserve_attrs: true,
                },
                &self.0,
            )?;
        }

        Ok(())
    }
}
