mod boolean;
pub mod error;
mod gimesh;
mod merge;
mod seperate;
mod slice;
mod vertex;

pub const DEFAULT_VERTEX_MERGE_DISTANCE: f32 = 0.0001;

pub use gimesh::GIMesh;
pub use vertex::Vertex;

pub use boolean::Boolean;

pub use merge::MergeSettings;
pub use seperate::SeperateOutput;

// ---- Deprecated ----
#[deprecated(
    note = "use [`GIMesh::merge_meshes`] or [`GIMesh::merge_vertices`]",
    since = "0.2.0"
)]
pub use merge::{merge_meshes, merge_vertices};
#[deprecated(note = "use [`GIMesh::seperate`]", since = "0.2.0")]
pub use seperate::seperate;
#[deprecated(note = "use [`GIMesh::slice`]", since = "0.2.0")]
pub use slice::slice;
