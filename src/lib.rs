mod boolean;
mod error;
mod gimesh;
mod merge;
mod seperate;
mod slice;
mod vertex;

pub const DEFAULT_VERTEX_MERGE_DISTANCE: f32 = 0.0001;

pub use boolean::Boolean;
pub use gimesh::GIMesh;
pub use merge::{merge_meshes, MergeSettings};
pub use seperate::{seperate, SeperateOutput};
pub use slice::slice;
pub use vertex::Vertex;
