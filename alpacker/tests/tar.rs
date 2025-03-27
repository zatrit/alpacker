use std::error::Error;

use alpacker::{Assets, Pack, pack::TarZstPack};
use rstest::{fixture, rstest};

const SAMPLES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../test-samples/");

type TestResult = Result<(), Box<dyn Error>>;

/// Fixture that loads assets from the test samples directory.
/// This runs once before all tests due to the `#[once]` attribute.
#[fixture]
#[once]
fn assets() -> Assets {
    Assets::load_from_dir(SAMPLES_DIR).unwrap()
}

/// Fixture that loads the "test" asset pack from the available assets.
/// Uses the `assets` fixture as a dependency.
#[fixture]
fn pack(assets: &Assets) -> TarZstPack {
    assets.load_pack("test").unwrap()
}

/// Test that verifies the correct behavior of `load_pack()`.
/// - Ensures an error is returned when trying to load a non-existent pack.
/// - Verifies that the "test" pack loads successfully and has no skipped files.
#[rstest]
fn test_load_pack(assets: &Assets) -> TestResult {
    assert!(
        assets.load_pack::<TarZstPack>("non_existent_pack").is_err(),
        "Expected error when loading a non-existent pack"
    );

    let pack = assets.load_pack::<TarZstPack>("test")?;
    assert!(
        pack.skipped().is_empty(),
        "Expected no skipped files, but some were skipped"
    );

    Ok(())
}

/// Test that verifies the correct behavior of `get()` for retrieving string assets.
/// - Ensures an error is returned when trying to retrieve a non-existent file.
/// - Checks that the content of `myfile.txt` matches the expected string.
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
