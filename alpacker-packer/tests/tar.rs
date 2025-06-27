mod common;
use common::*;

use testdir::testdir;

use alpacker::{Assets, MANIFEST_FILE, Pack};
use alpacker_packer::{AssetsBuilder, PackBuilder, TarZstPack, tar::Skipped};

#[test]
fn test_tar_zstd_pack() -> TestResult {
    // Prepare test environment
    let test_dir = testdir!();

    let pack = PackBuilder::new()?.copy_from(ASSETS_DIR)?;

    AssetsBuilder::new(&test_dir, "./")?
        .add_pack::<TarZstPack>("test", &pack)?
        .write_manifest(false)?;

    let manifest_path = test_dir.join(MANIFEST_FILE);
    assert!(
        manifest_path.exists(),
        "Manifest file not found at {manifest_path:?}"
    );

    // Test assets and pack loading
    let assets = Assets::load_from_dir(test_dir)?;

    let mut pack = assets.load_pack::<TarZstPack>("test")?;

    let skipped = pack.skipped();
    assert_eq!(
        skipped[0],
        Skipped::Manifest,
        "Expected the manifest to be skipped"
    );
    assert_eq!(skipped.len(), 1, "Expected only the manifest to be skipped");

    let data = pack.get::<String>("myfile.txt")?;
    assert_eq!(
        data, "Hello, World!\n",
        "File content does not match expected output"
    );

    Ok(())
}
