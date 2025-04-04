#![allow(unused)]

use std::error::Error;

pub type TestResult = Result<(), Box<dyn Error>>;

pub const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets/");
pub const IMAGE: &str = "cat.png";
