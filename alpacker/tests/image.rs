mod common;
use common::*;

use alpacker::{Pack, pack::TarZstPack};

use image::DynamicImage;
use rstest::rstest;

#[rstest]
fn test_image_load(mut pack: TarZstPack) -> TestResult {
    let image = pack.get::<DynamicImage>(IMAGE)?;
    assert_eq!(image.width(), 45, "Wrong image width");
    assert_eq!(image.height(), 27, "Wrong image height");

    Ok(())
}
