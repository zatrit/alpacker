use std::error::Error;
use testdir::testdir;

use alpacker::{Assets, Pack, TarZstPack};
use alpacker_packer::{AssetsBuilder, PackBuilder, transform::OxipngTransform};

const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../test-assets/");

#[test]
fn test_create_tar_zst() -> Result<(), Box<dyn Error>> {
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

    let manifest_path = test_dir.join("manifest.toml");
    assert!(
        manifest_path.exists(),
        "Manifest file not found at {manifest_path:?}"
    );

    let assets = Assets::load_from_dir(test_dir)?;

    assert!(
        assets.load_pack::<TarZstPack>("non_existent_pack").is_err(),
        "Expected error when loading a non-existent pack"
    );

    let mut pack = assets.load_pack::<TarZstPack>("test")?;
    assert!(
        pack.skipped().len() == 0,
        "Expected no skipped files, but some were skipped"
    );

    assert!(
        pack.get::<String>("non_existent_file.txt").is_err(),
        "Expected error when retrieving a non-existent file"
    );

    let data = pack.get::<String>("myfile.txt")?;
    assert_eq!(
        data, "Hello, World!\n",
        "File content does not match expected output"
    );

    Ok(())
}
