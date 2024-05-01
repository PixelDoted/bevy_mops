# Bevy MOps (Mesh Operations)

[![docs.rs](https://docs.rs/bevy_mops/badge.svg)](https://docs.rs/bevy_mops)
[![crates.io](https://img.shields.io/crates/v/bevy_mops)](https://crates.io/crates/bevy_mops)
[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)

## Getting started

```rust
// We convert from a "Bevy Mesh" to a `GIMesh` (Globally-positioned Index Mesh)
let mut a = GIMesh::from_mesh(mesh_a);
let b = GIMesh::from_mesh(mesh_b);

// slice `a` triangles by `b`
a.slice(&b);

// seperates `a` into `inside` and `outside` of `b`
let output = a.seperate(&b);

// Convert the `output` back to a "Bevy Mesh"
let output_mesh = output.inside.to_mesh().unwrap();

// ... Create a handle and apply it to an entity
```


## Why convert to `GIMesh`?
  
1. Mesh Attributes are converted to Vectors (e.g. `Vec3`)  
2. The Positions and Normals are converted from Local-space to World-space (and back), allowing position, rotation and scaling to effect the operations  

## Versions

| bevy  | bevy_mops |
|-------|-----------|
| 0.13  | 0.1       |

## LICENSE

all code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](./LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE))

at your option.
