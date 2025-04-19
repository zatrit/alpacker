# alpacker

**alpacker** is a modular asset packaging and loading library for Rust. It provides tools for building asset packs from file directories, transforming and compressing them, and later loading them back with minimal runtime cost.

Some comments *(not the code)* are written with [DeepSeek](https://deepseek.com) and [ChatGPT](https://chatgpt.com).

## Features

- Build asset packs using TAR or Zstandard compression.
- Apply transformations before packaging (e.g., PNG optimization using `oxipng`).
- Load assets such as strings, images, binary blobs, Aseprite metadata, and Raylib types.
- Generate and consume asset manifests with metadata and references to packaged files.
- Fully extensible via traits for custom packaging and asset types.

## Usage

### Building a TAR package

```rust
use alpacker_packer::{PackBuilder, MakePack, TarPack};
use std::fs::File;

// Creates a temporary workspace for packaging.
// "builder" works in a temporary directory, original files are untouched.
let builder = PackBuilder::new()
    .unwrap()
    .copy_from("./assets")
    .unwrap();

let file = File::create("./assets.tar").unwrap();
builder.write_pack::<TarPack>(file).unwrap();
```

### Applying transformations (e.g. better PNG compression)

```rust
use alpacker_packer::transform::OxipngTransform;

// Create the builder first, before applying transforms.
let mut transformer = OxipngTransform::default();
let builder = builder.transform(&mut transformer).unwrap();
```

### Building a compressed `.tar.zst` package

```rust
use alpacker_packer::TarZstPack;

// Create the builder first, before writing pack.
let file = File::create("assets.tar.zst").unwrap();
builder.write_pack::<TarZstPack>(file).unwrap();
```

### Creating a manifest

```rust
use alpacker_packer::{AssetsBuilder, TarPack};

// Creates an asset directory called "build" in the working directory.
// It also creates a "packs" directory, which will contain packs.
let manifest = AssetsBuilder::new("./build", "packs")
    .unwrap()
    .add_pack::<TarPack>("main", &builder) // Creates the package "main.tar" in the "./build/packs/" directory.
    .unwrap()
    .write_manifest() // Writes manifest to "./build/manifest.json"
    .unwrap();
```

### Loading an asset ``Pack``

```rust
use alpacker::{Assets, Asset, pack::TarPack};

let assets = Assets::load_from_dir("./assets").unwrap();
let mut pack = assets.load_pack::<TarPack>("main").unwrap();

let text: String = pack.get("file.txt").unwrap();
```

### Loading Aseprite sprites

```rust
use alpacker::data::aseprite::{Sprite, SpritesheetData};
use alpacker::data::image::ImageSprite;

let sprite: ImageSprite = pack.get("sprites/hero.json").unwrap();
```

### Loading Raylib audio

```rust
use alpacker::data::raylib::{PackRaylibExt, RaylibAsset};
use raylib::audio::RaylibAudio;

let mut audio = RaylibAudio::init_audio_device();
let music = pack.get_raylib::<raylib::audio::Music>(&mut audio, "music/title.ogg").unwrap();
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE.txt) (Apache-2.0).
