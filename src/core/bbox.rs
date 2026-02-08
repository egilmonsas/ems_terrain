use std::default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}
impl BBox {
    #[must_use]
    pub fn width(&self) -> f32 {
        self.x2 - self.x1
    }
    #[must_use]
    pub fn height(&self) -> f32 {
        self.y2 - self.y1
    }
    #[must_use]
    pub fn num_pixels_x(&self, resolution: f32) -> usize {
        (self.width() / resolution).ceil() as usize
    }
    #[must_use]
    pub fn num_pixels_y(&self, resolution: f32) -> usize {
        (self.height() / resolution).ceil() as usize
    }
}

impl default::Default for BBox {
    fn default() -> Self {
        Self {
            x1: 112_782.0,
            y1: 121_7012.0,
            x2: 112_971.0,
            y2: 121_7166.0,
        }
    }
}
