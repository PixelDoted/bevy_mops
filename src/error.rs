use bevy::utils::thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("The mesh has no vertices")]
    NoVertices,

    #[error("The mesh has no indices")]
    NoIndices,

    #[error("The mesh has no normals")]
    NoNormals,

    #[error("The vertex position was not float3x3")]
    VertexInvalidFormat,

    #[error("The normal was not float3x3")]
    NormalInvalidFormat,

    #[error("A vertex is missing an expected attribute")]
    VertexMissingAttribute,
}
