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
use alpacker::pack::TarPack;
use alpacker_packer::{PackBuilder, MakePack};
use std::fs::File;

let builder = PackBuilder::new("assets")
    .unwrap()
    .copy_from("assets_dir")
    .unwrap();

let file = File::create("assets.tar").unwrap();
builder.make_pack::<TarPack>(file).unwrap();
```

### Applying transformations (e.g. better PNG compression)

```rust
use alpacker_packer::transform::OxipngTransform;

let mut transformer = OxipngTransform::default();
let builder = builder.transform(&mut transformer).unwrap();
```

### Building a compressed `.tar.zst` package

```rust
use alpacker::pack::{TarPack, Zstd};

let file = File::create("assets.tar.zst").unwrap();
builder.make_pack::<Zstd<TarPack>>(file).unwrap();
```

### Creating a manifest

```rust
use alpacker_packer::AssetsBuilder;

let manifest = AssetsBuilder::new("build", "packs")
    .unwrap()
    .add_pack::<TarPack>("main", &builder)
    .unwrap()
    .write_manifest()
    .unwrap();
```

### Loading an asset ``Pack``

```rust
use alpacker::{Assets, Asset, pack::TarPack};

let assets = Assets::load_from_dir("build").unwrap();
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
