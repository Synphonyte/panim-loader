# PANIM Loader

[![Crates.io](https://img.shields.io/crates/v/panim-loader.svg)](https://crates.io/crates/panim-loader)
[![Docs](https://docs.rs/panim-loader/badge.svg)](https://docs.rs/panim-loader/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/synphonyte/panim-loader#license)
[![Build Status](https://github.com/synphonyte/panim-loader/actions/workflows/ci.yml/badge.svg)](https://github.com/synphonyte/panim-loader/actions/workflows/ci.yml)

<!-- cargo-rdme start -->

This is a loader for the PANIM format, which is a simple animation format for custom properties in Blender.
This is meant to be used in conjunction with the [PANIM Blender Exporter](https://github.com/Synphonyte/blender-panim-exporter).

Please also check there to see the details of the binary file format.

The primary use case for this file type is to export more animation data on top of what can be stored in GLTF files.

## Usage

```rust
use panim_loader::PropertiesAnimation;

let anims = PropertiesAnimation::from_file("assets/single_anim.panim").unwrap();
let value = anims.get_animation_value_at_time(&anims.animations[0], 10.0);
```

<!-- cargo-rdme end -->
