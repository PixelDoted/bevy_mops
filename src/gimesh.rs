use crate::{error::ConvertError, vertex::Vertex};
use bevy::{
    math::Affine3A,
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
        render_asset::RenderAssetUsages,
    },
};

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

// Conversion
impl GIMesh {
    /// from [`bevy::prelude::Mesh`] to [`GIMesh`]
    pub fn from_mesh(mesh: &Mesh, model: Affine3A) -> Result<Self, ConvertError> {
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .ok_or(ConvertError::NoVertices)?;
        let normals = mesh
            .attribute(Mesh::ATTRIBUTE_NORMAL)
            .ok_or(ConvertError::NoNormals)?;
        let uvs_0 = mesh.attribute(Mesh::ATTRIBUTE_UV_0);
        let uvs_1 = mesh.attribute(Mesh::ATTRIBUTE_UV_1);
        let tangents = mesh.attribute(Mesh::ATTRIBUTE_TANGENT);
        let colors = mesh.attribute(Mesh::ATTRIBUTE_COLOR);
        let joint_weights = mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT);
        let joint_indices = mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX);

        let mut output = Self {
            indices: match mesh.indices().ok_or(ConvertError::NoIndices)? {
                Indices::U16(v) => v.iter().map(|v| *v as u32).collect(),
                Indices::U32(v) => v.clone(),
            },
            vertices: Vec::with_capacity(positions.len()),
            inverse_model: model.inverse(),
        };

        for i in 0..output.vertices.capacity() {
            let vertex = Vertex {
                pos: match positions {
                    VertexAttributeValues::Float32x3(v) => model.transform_point3a(v[i].into()),
                    _ => return Err(ConvertError::VertexInvalidFormat),
                },
                normal: match normals {
                    VertexAttributeValues::Float32x3(v) => model.transform_vector3a(v[i].into()),
                    _ => return Err(ConvertError::NormalInvalidFormat),
                },
                uv0: match uvs_0 {
                    Some(VertexAttributeValues::Float32x2(v)) => Some(v[i].into()),
                    _ => None,
                },
                uv1: match uvs_1 {
                    Some(VertexAttributeValues::Float32x2(v)) => Some(v[i].into()),
                    _ => None,
                },
                tangent: match tangents {
                    Some(VertexAttributeValues::Float32x4(v)) => Some(v[i].into()),
                    _ => None,
                },
                color: match colors {
                    Some(VertexAttributeValues::Float32x4(v)) => Some(v[i].into()),
                    _ => None,
                },
                joint_weight: match joint_weights {
                    Some(VertexAttributeValues::Float32x4(v)) => Some(v[i].into()),
                    _ => None,
                },
                joint_index: match joint_indices {
                    Some(VertexAttributeValues::Uint16x4(v)) => Some(v[i].into()),
                    _ => None,
                },
            };

            output.vertices.push(vertex);
        }

        Ok(output)
    }

    /// to [`bevy::prelude::Mesh`] to [`GIMesh`]
    pub fn to_mesh(self) -> Result<Mesh, ConvertError> {
        if self.indices.is_empty() {
            return Err(ConvertError::NoIndices);
        }
        if self.vertices.is_empty() {
            return Err(ConvertError::NoVertices);
        }

        let indices = Indices::U32(self.indices);

        let vert_count = self.vertices.len();
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vert_count);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vert_count);
        let mut uvs0: Option<Vec<[f32; 2]>> = self.vertices[0]
            .uv0
            .is_some()
            .then_some(Vec::with_capacity(vert_count));
        let mut uvs1: Option<Vec<[f32; 2]>> = self.vertices[0]
            .uv1
            .is_some()
            .then_some(Vec::with_capacity(vert_count));
        let mut tangents: Option<Vec<[f32; 4]>> = self.vertices[0]
            .tangent
            .is_some()
            .then_some(Vec::with_capacity(vert_count));
        let mut colors: Option<Vec<[f32; 4]>> = self.vertices[0]
            .color
            .is_some()
            .then_some(Vec::with_capacity(vert_count));
        let mut joint_weights: Option<Vec<[f32; 4]>> = self.vertices[0]
            .joint_weight
            .is_some()
            .then_some(Vec::with_capacity(vert_count));
        let mut joint_indices: Option<Vec<[u16; 4]>> = self.vertices[0]
            .joint_index
            .is_some()
            .then_some(Vec::with_capacity(vert_count));

        for v in self.vertices {
            positions.push(self.inverse_model.transform_point3a(v.pos).into());
            normals.push(self.inverse_model.transform_vector3a(v.pos).into());

            if let Some(values) = &mut uvs0 {
                values.push(v.uv0.ok_or(ConvertError::VertexMissingAttribute)?.into());
            }
            if let Some(values) = &mut uvs1 {
                values.push(v.uv1.ok_or(ConvertError::VertexMissingAttribute)?.into());
            }
            if let Some(values) = &mut tangents {
                values.push(
                    v.tangent
                        .ok_or(ConvertError::VertexMissingAttribute)?
                        .into(),
                );
            }
            if let Some(values) = &mut colors {
                values.push(v.color.ok_or(ConvertError::VertexMissingAttribute)?.into());
            }
            if let Some(values) = &mut joint_weights {
                values.push(
                    v.joint_weight
                        .ok_or(ConvertError::VertexMissingAttribute)?
                        .into(),
                );
            }
            if let Some(values) = &mut joint_indices {
                values.push(
                    v.joint_index
                        .ok_or(ConvertError::VertexMissingAttribute)?
                        .into(),
                );
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all())
            .with_inserted_indices(indices);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        if let Some(values) = uvs0.take() {
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, values);
        }
        if let Some(values) = uvs1.take() {
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_1, values);
        }
        if let Some(values) = tangents.take() {
            mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, values);
        }
        if let Some(values) = colors.take() {
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, values);
        }
        if let Some(values) = joint_weights.take() {
            mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, values);
        }
        if let Some(values) = joint_indices.take() {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_JOINT_INDEX,
                VertexAttributeValues::Uint16x4(values),
            );
        }

        Ok(mesh)
    }
}
