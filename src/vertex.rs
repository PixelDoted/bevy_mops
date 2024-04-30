use std::hash::Hasher;

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

impl std::hash::Hash for Vertex {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(self.pos.x.to_bits());
        hasher.write_u32(self.pos.y.to_bits());
        hasher.write_u32(self.pos.z.to_bits());

        hasher.write_u32(self.normal.x.to_bits());
        hasher.write_u32(self.normal.y.to_bits());
        hasher.write_u32(self.normal.z.to_bits());

        let uv0 = self.uv0.unwrap_or(Vec2::ZERO);
        let uv1 = self.uv1.unwrap_or(Vec2::ZERO);
        let tangent = self.tangent.unwrap_or(Vec4::ZERO);
        let color = self.color.unwrap_or(Vec4::ZERO);
        let joint_weight = self.joint_weight.unwrap_or(Vec4::ZERO);
        let joint_index = self.joint_index.unwrap_or(U16Vec4::ZERO);

        hasher.write_u32(uv0.x.to_bits());
        hasher.write_u32(uv0.y.to_bits());

        hasher.write_u32(uv1.x.to_bits());
        hasher.write_u32(uv1.y.to_bits());

        hasher.write_u32(tangent.x.to_bits());
        hasher.write_u32(tangent.y.to_bits());
        hasher.write_u32(tangent.z.to_bits());
        hasher.write_u32(tangent.w.to_bits());

        hasher.write_u32(color.x.to_bits());
        hasher.write_u32(color.y.to_bits());
        hasher.write_u32(color.z.to_bits());
        hasher.write_u32(color.w.to_bits());

        hasher.write_u32(joint_weight.x.to_bits());
        hasher.write_u32(joint_weight.y.to_bits());
        hasher.write_u32(joint_weight.z.to_bits());
        hasher.write_u32(joint_weight.w.to_bits());

        hasher.write_u16(joint_index.x);
        hasher.write_u16(joint_index.y);
        hasher.write_u16(joint_index.z);
        hasher.write_u16(joint_index.w);
    }
}
