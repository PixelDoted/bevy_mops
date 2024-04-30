mod conversion;
mod ops;

use crate::vertex::Vertex;
use bevy::math::Affine3A;

/// A Globally-positioned Index Mesh
#[derive(Clone)]
pub struct GIMesh {
    /// See [`add_index`], [`set_index`], [`index`] and [`index_count`]
    pub indices: Vec<u32>,

    /// See [`add_vertex`], [`set_vertex`], [`vertex`] and [`vertex_count`]
    pub vertices: Vec<Vertex>,

    pub inverse_model: Affine3A,
}

impl GIMesh {
    pub fn tri_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Returns 3 indices
    pub fn tri(&self, t: usize) -> [(u32, usize); 3] {
        let i = t * 3;
        [
            (self.indices[i], i),
            (self.indices[i + 1], i + 1),
            (self.indices[i + 2], i + 2),
        ]
    }
}

impl GIMesh {
    /// Adds a vertex to the mesh
    ///
    /// NOTE: Assumes the vertex is unique
    pub fn add_vertex(&mut self, v: Vertex) -> u32 {
        let index = self.vertices.len();
        self.vertices.push(v);
        index as u32
    }

    /// Sets the vertex
    pub fn set_vertex(&mut self, index: u32, v: Vertex) {
        self.vertices[index as usize] = v;
    }

    /// Gets the vertex at `index`
    pub fn vertex(&self, index: u32) -> &Vertex {
        &self.vertices[index as usize]
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }

    pub fn set_index(&mut self, i: usize, index: u32) {
        self.indices[i] = index;
    }

    /// Gets the index of a vertex
    pub fn index(&self, i: usize) -> u32 {
        self.indices[i]
    }

    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}
