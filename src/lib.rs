pub mod core;
pub mod export;
pub mod processing;
pub mod providers;

// Prelude for easy importing in your Tauri main.rs
pub mod prelude {
    pub use crate::core::bbox::BBox;
    pub use crate::core::mesh::Mesh;
    pub use crate::core::vertex::Vertex;
    pub use crate::export::ifc::IfcWriter;
    pub use crate::processing::{surface::gaussian_blur_mesh, CompressionParams, FilterParams};
    pub use crate::providers::{geonorge::TerrainProvider, DataProvider};
}
