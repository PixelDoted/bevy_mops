mod error;
mod gimesh;
mod merge;
mod seperate;
mod slice;
mod vertex;

pub use gimesh::GIMesh;
pub use merge::{merge_meshes, MergeSettings};
pub use seperate::{seperate, SeperateOutput};
pub use slice::slice;
pub use vertex::Vertex;
