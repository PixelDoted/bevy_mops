# Bevy MOps (Mesh Operations)

## Getting started

```rust
// We convert from a "Bevy Mesh" to a `GIMesh` (Globally-positioned Index Mesh)
let mut a = GIMesh::from_mesh(mesh_a);
let b = GIMesh::from_mesh(mesh_b);

// slice `a` triangles by `b`
slice(&mut a, &b);

// seperates `a` into `inside` and `outside` of `b`
let output = seperate(&a, &b);

// Convert the `output` back to a "Bevy Mesh"
let output_mesh = output.inside.to_mesh().unwrap();

// ... Create a handle and apply it to an entity
```


## Why convert to `GIMesh`?
  
1. Mesh Attributes are converted to Vectors (e.g. `Vec3`)  
2. The Positions and Normals are converted from Local-space to World-space (and back), allowing position, rotation and scaling to effect the operations  
