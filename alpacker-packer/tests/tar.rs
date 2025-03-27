use std::error::Error;
use testdir::testdir;

use alpacker::{pack::TarZstPack, Assets, Pack, MANIFEST_FILE};
use alpacker_packer::{AssetsBuilder, PackBuilder, transform::OxipngTransform};

const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../test-assets/");

#[test]
fn test_create() -> Result<(), Box<dyn Error>> {
    // Prepare test environment
    let test_dir = testdir!();

    // Write manifest and packs
    let mut oxipng = OxipngTransform(oxipng::Options::max_compression());

    let pack = PackBuilder::new("test")?
        .copy_from(ASSETS_DIR)?
        .transform(&mut oxipng)?;

    AssetsBuilder::new(&test_dir, "./")?
        .add_pack::<TarZstPack>("test", &pack)?
        .write_manifest()?;

    let manifest_path = test_dir.join(MANIFEST_FILE);
    assert!(
        manifest_path.exists(),
        "Manifest file not found at {manifest_path:?}"
    );

    // Test assets and pack loading
    let assets = Assets::load_from_dir(test_dir)?;

    let mut pack = assets.load_pack::<TarZstPack>("test")?;
    assert!(
        pack.skipped().len() == 0,
        "Expected no skipped files, but some were skipped"
    );

    let data = pack.get::<String>("myfile.txt")?;
    assert_eq!(
        data, "Hello, World!\n",
        "File content does not match expected output"
    );

    Ok(())
}
