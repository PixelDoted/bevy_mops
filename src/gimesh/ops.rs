use super::GIMesh;

impl GIMesh {
    /// Slices the triangles of `self` by the triangles of `slicer`
    pub fn slice(&mut self, slicer: &Self) -> &mut Self {
        crate::slice(self, slicer);
        self
    }

    /// Seperates `self` into `inside` and `outside` of `other`
    ///
    /// NOTE: this doesn't slice triangles
    pub fn seperate(&self, other: &Self) -> crate::SeperateOutput {
        crate::seperate(self, other)
    }

    /// Merges `other` into `self`
    pub fn merge_with(&mut self, other: &Self, settings: &crate::MergeSettings) -> &mut Self {
        crate::merge_meshes(self, other, settings);
        self
    }

    /// Returns a new [`GIMesh`] where every vertex within [`distance`] of another vertex are merged
    pub fn merge_vertices(&self, distance: f32) -> GIMesh {
        crate::merge_vertices(self, distance)
    }

    /// Inverts the normals
    pub fn invert_normals(&mut self) -> &mut Self {
        for v in &mut self.vertices {
            v.normal = -v.normal;
        }

        self
    }

    /// Flips the vertex windings
    pub fn flip_windings(&mut self) -> &mut Self {
        for t in 0..self.tri_count() {
            let tri = self.tri(t);
            self.set_index(tri[0].1, tri[2].0);
            self.set_index(tri[2].1, tri[0].0);
        }

        self
    }
}
