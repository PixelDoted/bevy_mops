use crate::{GIMesh, MergeSettings, SeperateOutput, DEFAULT_VERTEX_MERGE_DISTANCE};

pub struct Boolean<'a> {
    pub a: &'a GIMesh,
    pub b: &'a GIMesh,
    pub vertex_merge_distance: f32,
}

impl<'a> Boolean<'a> {
    /// Initializes [`Boolean`] with a default [`vertex_merge_distance`]
    pub fn new(a: &'a GIMesh, b: &'a GIMesh) -> Self {
        Self {
            a,
            b,
            vertex_merge_distance: DEFAULT_VERTEX_MERGE_DISTANCE,
        }
    }

    pub fn intersection(&self) -> GIMesh {
        let [mut ra, rb] = self.slice_and_seperate();
        ra.inside.invert_normals().flip_windings().merge_with(
            &rb.inside,
            &MergeSettings {
                merge_distance: self.vertex_merge_distance,
                invert_b_normals: true,
            },
        );

        ra.inside
    }

    pub fn difference(&self) -> GIMesh {
        let [mut ra, rb] = self.slice_and_seperate();
        ra.outside.merge_with(
            &rb.inside,
            &MergeSettings {
                merge_distance: self.vertex_merge_distance,
                invert_b_normals: true,
            },
        );

        ra.outside
    }

    pub fn union(&self) -> GIMesh {
        let [mut ra, rb] = self.slice_and_seperate();
        ra.outside.merge_with(
            &rb.outside,
            &MergeSettings {
                merge_distance: self.vertex_merge_distance,
                invert_b_normals: false,
            },
        );

        ra.outside
    }

    /// Useful for custom boolean operations
    pub fn slice_and_seperate(&self) -> [SeperateOutput; 2] {
        let mut aa = self.a.clone();
        let mut bb = self.b.clone();

        [
            aa.slice(self.b).seperate(self.b),
            bb.slice(self.a).seperate(self.a),
        ]
    }
}
