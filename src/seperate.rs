use bevy::math::Vec3A;

use crate::{GIMesh, Vertex};

/// Seperates `a` into `inside` and `outside` of `b`
///
/// NOTE: this doesn't slice triangles
pub fn seperate(a: &GIMesh, b: &GIMesh) -> SeperateOutput {
    let mut output = SeperateOutput {
        inside: GIMesh {
            indices: Vec::with_capacity(a.index_count()),
            vertices: Vec::with_capacity(a.vertex_count() as usize),
            inverse_model: a.inverse_model,
        },
        outside: GIMesh {
            indices: Vec::with_capacity(a.index_count()),
            vertices: Vec::with_capacity(a.vertex_count() as usize),
            inverse_model: a.inverse_model,
        },
    };

    for ta in 0..a.tri_count() {
        let a_tri = a.tri(ta);
        let a_verts = [
            a.vertex(a_tri[0].0),
            a.vertex(a_tri[1].0),
            a.vertex(a_tri[2].0),
        ];

        let a_center = (a_verts[0].pos + a_verts[1].pos + a_verts[2].pos) / 3.0;
        let a_normal = (a_verts[0].normal + a_verts[1].normal + a_verts[2].normal) / 3.0;

        let mut hits = 0;
        for tb in 0..b.tri_count() {
            let b_tri = b.tri(tb);
            let b_verts = [
                b.vertex(b_tri[0].0),
                b.vertex(b_tri[1].0),
                b.vertex(b_tri[2].0),
            ];

            if ray_triangle(a_center, a_normal, b_verts) {
                hits += 1;
            }
        }

        // Add Triangle to it's respective mesh
        let mesh = if hits % 2 == 0 {
            // Outside
            &mut output.outside
        } else {
            // Inside
            &mut output.inside
        };

        // FIXME: This will readd vertices already added to `mesh`
        let a = mesh.add_vertex(a_verts[0].clone());
        let b = mesh.add_vertex(a_verts[1].clone());
        let c = mesh.add_vertex(a_verts[2].clone());

        mesh.add_index(a);
        mesh.add_index(b);
        mesh.add_index(c);
    }

    output
}

fn ray_triangle(ro: Vec3A, rv: Vec3A, tri: [&Vertex; 3]) -> bool {
    let edge1 = tri[1].pos - tri[0].pos;
    let edge2 = tri[2].pos - tri[0].pos;
    let ray_cross_e2 = rv.cross(edge2);
    let det = edge1.dot(ray_cross_e2);
    if det > -f32::EPSILON && det < f32::EPSILON {
        return false;
    }

    let inv_det = 1.0 / det;
    let s = ro - tri[0].pos;
    let u = inv_det * s.dot(ray_cross_e2);
    if !(u >= 0.0 && u <= 1.0) {
        return false;
    }

    let s_cross_e1 = s.cross(edge1);
    let v = inv_det * rv.dot(s_cross_e1);
    if v < 0.0 || u + v > 1.0 {
        return false;
    }

    inv_det * edge2.dot(s_cross_e1) > f32::EPSILON
}

pub struct SeperateOutput {
    #[doc(alias = "intersection")]
    pub inside: GIMesh,

    #[doc(alias = "difference")]
    pub outside: GIMesh,
}
