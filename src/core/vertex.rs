use bytemuck::{Pod, Zeroable};
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub struct Vertex {
    pub position: [f32; 3],
}
impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            position: [x, y, z],
        }
    }
}
