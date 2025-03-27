use std::error::Error;

use alpacker::{Assets, Pack, pack::TarZstPack};
use rstest::{fixture, rstest};

const SAMPLES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../test-samples/");

type TestResult = Result<(), Box<dyn Error>>;

#[fixture]
#[once]
fn assets() -> Assets {
    Assets::load_from_dir(SAMPLES_DIR).unwrap()
}

#[fixture]
fn pack(assets: &Assets) -> TarZstPack {
    assets.load_pack("test").unwrap()
}

#[rstest]
fn test_load_pack(assets: &Assets) -> TestResult {
    assert!(
        assets.load_pack::<TarZstPack>("non_existent_pack").is_err(),
        "Expected error when loading a non-existent pack"
    );

    let pack = assets.load_pack::<TarZstPack>("test")?;
    assert!(
        pack.skipped().len() == 0,
        "Expected no skipped files, but some were skipped"
    );

    Ok(())
}

#[rstest]
fn test_load_string(mut pack: TarZstPack) -> TestResult {
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
