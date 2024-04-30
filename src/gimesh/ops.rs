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
}
