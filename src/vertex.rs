use bevy::{
    math::{U16Vec4, Vec3A},
    prelude::*,
};

/// Contains all the Vertex data from a [`Mesh`]
#[derive(Clone)]
pub struct Vertex {
    pub pos: Vec3A,
    pub normal: Vec3A,
    pub uv0: Option<Vec2>,
    pub uv1: Option<Vec2>,
    pub tangent: Option<Vec4>,
    pub color: Option<Vec4>,
    pub joint_weight: Option<Vec4>,
    pub joint_index: Option<U16Vec4>,
}

impl Vertex {
    /// Interpolates the value between [`self`] and [`other`]
    pub fn lerp(&mut self, other: &Vertex, s: f32) {
        self.pos = self.pos.lerp(other.pos, s);
        self.normal = self.normal.lerp(other.normal, s);

        if let (Some(a), Some(b)) = (&mut self.uv0, other.uv0) {
            *a = a.lerp(b, s);
        }

        if let (Some(a), Some(b)) = (&mut self.uv1, other.uv1) {
            *a = a.lerp(b, s);
        }

        if let (Some(a), Some(b)) = (&mut self.tangent, other.tangent) {
            *a = a.lerp(b, s);
        }

        if let (Some(a), Some(b)) = (&mut self.color, other.color) {
            *a = a.lerp(b, s);
        }

        if let (Some(a), Some(b)) = (&mut self.joint_weight, other.joint_weight) {
            *a = a.lerp(b, s);
        }

        // NOTE: [`joint_index`] can't be interpolated
    }
}
