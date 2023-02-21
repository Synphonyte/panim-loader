# PANIM Loader

This is a loader for the PANIM format, which is a simple animation format for custom properties in Blender.
This is meant to be used in conjunction with the [PANIM Blender Exporter](https://github.com/Synphonyte/blender-panim-exporter).

Please also check there to see the details of the binary file format.

The primary use case for this file type is to export more animation data on top of what can be stored in GLTF files.

## Usage

```rust
use panim_loader::PropertiesAnimation;

let anim = PropertiesAnimation::from_file("assets/example.panim").unwrap()
```