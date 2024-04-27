use bevy::utils::{HashMap, HashSet};

use crate::GIMesh;

pub struct MergeSettings {
    pub merge_distance: f32,
    pub invert_b_normals: bool,
}

impl Default for MergeSettings {
    fn default() -> Self {
        Self {
            merge_distance: 0.0001,
            invert_b_normals: false,
        }
    }
}

/// Merges `b` into `a`
///
/// [`distance`] is the merge distance between vertices
pub fn merge_meshes(a: &mut GIMesh, b: &GIMesh, settings: &MergeSettings) {
    let distance = settings.merge_distance * settings.merge_distance;

    for t in 0..b.tri_count() {
        let tri = b.tri(t);
        let verts = [b.vertex(tri[0].0), b.vertex(tri[1].0), b.vertex(tri[2].0)];

        let mut ivs = [None, None, None];
        for ai in 0..a.index_count() {
            let aindex = a.index(ai);
            let av = a.vertex(aindex);

            for i in 0..3 {
                if ivs[i].is_some() {
                    continue;
                }

                if verts[i].pos.distance_squared(av.pos) < distance {
                    ivs[i] = Some(aindex);
                }
            }

            if ivs[0].is_some() && ivs[1].is_some() && ivs[2].is_some() {
                break;
            }
        }

        let i1 = ivs[0].unwrap_or_else(|| {
            let mut v = verts[0].clone();
            if settings.invert_b_normals {
                v.normal = -v.normal;
            }

            a.add_vertex(v)
        });
        let i2 = ivs[1].unwrap_or_else(|| {
            let mut v = verts[1].clone();
            if settings.invert_b_normals {
                v.normal = -v.normal;
            }

            a.add_vertex(v)
        });
        let i3 = ivs[2].unwrap_or_else(|| {
            let mut v = verts[2].clone();
            if settings.invert_b_normals {
                v.normal = -v.normal;
            }

            a.add_vertex(v)
        });

        if settings.invert_b_normals {
            a.add_index(i3);
            a.add_index(i2);
            a.add_index(i1);
        } else {
            a.add_index(i1);
            a.add_index(i2);
            a.add_index(i3);
        }
    }
}
