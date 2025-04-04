#![allow(unused)]

use std::error::Error;

use alpacker::{Assets, pack::TarZstPack};
use rstest::fixture;

pub type TestResult = Result<(), Box<dyn Error>>;

pub const SAMPLES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/samples/");

pub const IMAGE: &str = "cat.png";

/// Fixture that loads assets from the test samples directory.
/// This runs once before all tests due to the `#[once]` attribute.
#[fixture]
#[once]
pub fn assets() -> Assets {
    Assets::load_from_dir(SAMPLES_DIR).unwrap()
}

/// Fixture that loads the "test" asset pack from the available assets.
/// Uses the `assets` fixture as a dependency.
#[fixture]
pub fn pack(assets: &Assets) -> TarZstPack {
    assets.load_pack("test").unwrap()
}
