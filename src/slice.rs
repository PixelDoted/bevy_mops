use bevy::{math::Vec3A, prelude::*};

use crate::{gimesh::GIMesh, vertex::Vertex};

/// Slices `slicee` triangles that are intersecting `slicer` triangles
pub fn slice(slicee: &mut GIMesh, slicer: &GIMesh) {
    for slicer_i in 0..slicer.tri_count() {
        let slicer_indices = slicer.tri(slicer_i);
        let slicer_verts = [
            slicer.vertex(slicer_indices[0].0),
            slicer.vertex(slicer_indices[1].0),
            slicer.vertex(slicer_indices[2].0),
        ];

        let plane_normal =
            (slicer_verts[0].normal + slicer_verts[1].normal + slicer_verts[2].normal) / 3.0;
        let plane_center = (slicer_verts[0].pos + slicer_verts[1].pos + slicer_verts[2].pos) / 3.0;
        let plane = Plane::new(plane_normal, -plane_normal.dot(plane_center));

        // Find the minimum and maximum x, y and z values for AABB
        let slicer_min = slicer_verts[0]
            .pos
            .min(slicer_verts[1].pos)
            .min(slicer_verts[2].pos);
        let slicer_max = slicer_verts[0]
            .pos
            .max(slicer_verts[1].pos)
            .max(slicer_verts[2].pos);

        for t in 0..slicee.tri_count() {
            let indices = slicee.tri(t);
            let verts = [
                slicee.vertex(indices[0].0),
                slicee.vertex(indices[1].0),
                slicee.vertex(indices[2].0),
            ];

            // Find the minimum and maximum x, y and z values for AABB
            let min = verts[0].pos.min(verts[1].pos).min(verts[2].pos);
            let max = verts[0].pos.max(verts[1].pos).max(verts[2].pos);

            // Perform an AABB Check
            if min.x <= slicer_max.x
                && max.x >= slicer_min.x
                && min.y <= slicer_max.y
                && max.y >= slicer_min.y
                && min.z <= slicer_max.z
                && max.z >= slicer_min.z
            {
                slice_triangle(&plane, slicee, indices);
            }
        }
    }
}

#[derive(Clone)]
struct Plane {
    pub p: Vec4,
    pub n: Vec3A,
    pub d: f32,
}

impl Plane {
    pub fn new(normal: Vec3A, d: f32) -> Self {
        Self {
            p: normal.extend(d),
            n: normal,
            d,
        }
    }
}

enum SliceVertex {
    Index(u32),
    Vertex(Vertex),
}

impl SliceVertex {
    pub fn apply(&self, mesh: &mut GIMesh) -> u32 {
        match self {
            Self::Index(i) => *i,
            Self::Vertex(v) => mesh.add_vertex(v.clone()),
        }
    }
}

fn slice_triangle(plane: &Plane, mesh: &mut GIMesh, indices: [(u32, usize); 3]) {
    let sides = [
        plane.p.dot(mesh.vertex(indices[0].0).pos.extend(1.0)),
        plane.p.dot(mesh.vertex(indices[1].0).pos.extend(1.0)),
        plane.p.dot(mesh.vertex(indices[2].0).pos.extend(1.0)),
    ];

    if (sides[0] >= -f32::EPSILON && sides[1] >= -f32::EPSILON && sides[2] >= -f32::EPSILON)
        || (sides[0] <= f32::EPSILON && sides[1] <= f32::EPSILON && sides[2] <= f32::EPSILON)
    {
        return;
    }

    let mut above = Vec::with_capacity(4);
    let mut below = Vec::with_capacity(4);

    for i in 0..3 {
        let j = (i + 1) % 3;

        if sides[i] >= -f32::EPSILON {
            above.push(SliceVertex::Index(indices[i].0));
        }

        if sides[i] <= f32::EPSILON {
            below.push(SliceVertex::Index(indices[i].0));
        }

        if (sides[j] >= f32::EPSILON && sides[i] <= -f32::EPSILON)
            || (sides[i] >= f32::EPSILON && sides[j] <= -f32::EPSILON)
        {
            let vj = mesh.vertex(indices[j].0);
            let vi = mesh.vertex(indices[i].0);
            let s = 1.0 - (plane.d + plane.n.dot(vi.pos)) / plane.n.dot(vi.pos - vj.pos);

            let mut point = vj.clone();
            point.lerp(vi, s);

            above.push(SliceVertex::Vertex(point.clone()));
            below.push(SliceVertex::Vertex(point));
        }
    }

    let mut original_used = false;
    add_vertices_to_mesh(&mut above, mesh, &indices, &mut original_used);
    add_vertices_to_mesh(&mut below, mesh, &indices, &mut original_used);
}

fn add_vertices_to_mesh(
    vertices: &mut [SliceVertex],
    mesh: &mut GIMesh,
    indices: &[(u32, usize); 3],
    original_used: &mut bool,
) {
    let mut vi = 0;
    while vertices.len() >= 3 + vi {
        vi += 1;
        let a = &vertices[0];
        let b = &vertices[vi];
        let c = &vertices[vi + 1];

        let index_map = [a.apply(mesh), b.apply(mesh), c.apply(mesh)];

        if !*original_used {
            mesh.set_index(indices[0].1, index_map[0]);
            mesh.set_index(indices[1].1, index_map[1]);
            mesh.set_index(indices[2].1, index_map[2]);
            *original_used = true;
        } else {
            mesh.add_index(index_map[0]);
            mesh.add_index(index_map[1]);
            mesh.add_index(index_map[2]);
        }
    }
}
