use std::{io, path::Path};

use glob::glob;

use crate::Transform;

pub struct OxipngTransform(pub oxipng::Options);

impl Transform for OxipngTransform {
    fn transform(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref().join("*.png");
        for png_file in glob(path.to_str().unwrap()).unwrap() {

        };

        Ok(())
    }
}
