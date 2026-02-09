pub mod surface;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CompressionParams {
    pub compression_factor: f32, // 0.0 - 1.0 (1.0 = no compression)
}
impl Default for CompressionParams {
    fn default() -> Self {
        CompressionParams {
            compression_factor: 0.5,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FilterParams {
    pub radius: f32,   // in metres
    pub sigma: f32,    // strength of filter
    pub passes: usize, // number of passes
}
impl Default for FilterParams {
    fn default() -> Self {
        FilterParams {
            radius: 1.0,
            sigma: 0.5,
            passes: 2,
        }
    }
}
impl FilterParams {
    pub fn radius_in_px(&self, resolution: f32) -> usize {
        (self.radius / resolution).ceil() as usize
    }
}
