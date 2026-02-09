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
    pub use crate::processing::surface::{gaussian_blur_mesh, PostProcessParams};
    pub use crate::providers::{geonorge::TerrainProvider, DataProvider};
    pub use crate::GenerateRequest;
}

use crate::{export::ProjectMetadata, prelude::*, processing::surface::PostProcessParams};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GenerateRequest {
    pub bbox: BBox,
    pub resolution: f32,
    pub crs: usize,
    pub project_metadata: ProjectMetadata,
    pub post_process_params: PostProcessParams,
}
