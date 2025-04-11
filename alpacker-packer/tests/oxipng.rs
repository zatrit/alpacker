mod common;
use common::*;

use alpacker_packer::{PackBuilder, transform::OxipngTransform};
use std::{fs, path::Path};

#[test]
fn test_oxipng_optimization() -> TestResult {
    // Write manifest and packs
    let mut oxipng = OxipngTransform(oxipng::Options::max_compression());

    let pack = PackBuilder::new("test")?
        .copy_from(ASSETS_DIR)?
        .transform(&mut oxipng)?;

    let old_size = fs::metadata(Path::new(ASSETS_DIR).join(IMAGE))?.len();
    let new_size = fs::metadata(pack.work_dir().join(IMAGE))?.len();

    assert!(
        new_size < old_size,
        "Oxipng optimization doesn't work ({old_size} <= {new_size})"
    );

    Ok(())
}
