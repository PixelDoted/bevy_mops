use crate::{GIMesh, DEFAULT_VERTEX_MERGE_DISTANCE};

/// Returns a new [`GIMesh`] where every vertex within [`distance`] of another vertex are merged
pub fn merge_vertices(mesh: &GIMesh, distance: f32) -> GIMesh {
    let dist_sqr = distance * distance;
    let mut output = GIMesh {
        indices: Vec::with_capacity(mesh.index_count()),
        vertices: Vec::with_capacity(mesh.vertex_count() as usize),
        inverse_model: mesh.inverse_model,
    };

    for ai in &mesh.indices {
        let v = mesh.vertex(*ai);
        let mut i = None;

        for bv in 0..output.vertex_count() {
            if v.pos.distance_squared(output.vertex(bv).pos) < dist_sqr {
                i = Some(bv);
                break;
            }
        }

        if i.is_none() {
            i = Some(output.add_vertex(v.clone()));
        }

        output.add_index(i.unwrap());
    }

    output
}

pub struct MergeSettings {
    /// Merge distance between vertices
    pub merge_distance: f32,

    /// If `true` b's normals are inverted when added
    pub invert_b_normals: bool,
}

impl Default for MergeSettings {
    fn default() -> Self {
        Self {
            merge_distance: DEFAULT_VERTEX_MERGE_DISTANCE,
            invert_b_normals: false,
        }
    }
}

/// Merges `b` into `a`
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
