use crate::prelude::Vertex;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PostProcessParams {
    pub compression_factor: f32, // 0.0 - 1.0 (1.0 = no compression)
    pub radius: f32,             // in metres
    pub sigma: f32,              // strength of filter
    pub passes: usize,           // number of passes
}
impl Default for PostProcessParams {
    fn default() -> Self {
        PostProcessParams {
            compression_factor: 0.5,
            radius: 1.0,
            sigma: 0.5,
            passes: 2,
        }
    }
}
impl PostProcessParams {
    pub fn radius_in_px(&self, resolution: f32) -> usize {
        (self.radius / resolution).ceil() as usize
    }
}

fn gaussian_kernel(radius: usize, sigma: f32) -> Vec<Vec<f32>> {
    let size = 2 * radius + 1;
    let mut kernel = vec![vec![0.0; size]; size];
    let mut sum = 0.0;

    let r = radius as i32;

    for y in -r..=r {
        for x in -r..=r {
            let value = (-((x * x + y * y) as f32) / (2.0 * sigma * sigma)).exp();
            kernel[(y + r) as usize][(x + r) as usize] = value;
            sum += value;
        }
    }

    // Normalize
    for row in kernel.iter_mut() {
        for v in row.iter_mut() {
            *v /= sum;
        }
    }

    kernel
}

// Filer
pub fn apply_gaussian_blur(
    vertices: &mut [Vertex],
    width: usize,
    height: usize,
    resolution: f32,
    gauss_params: &PostProcessParams,
) {
    let kernel = gaussian_kernel(gauss_params.radius_in_px(resolution), gauss_params.sigma);
    let radius_px = gauss_params.radius_in_px(resolution);

    let mut z_buffer: Vec<f32> = vertices.iter().map(|v| v.position[2]).collect();

    for _ in 0..gauss_params.passes {
        let z_original = z_buffer.clone();

        for y in radius_px..height - radius_px {
            for x in radius_px..width - radius_px {
                let mut sum = 0.0;

                for (ky, row) in kernel.iter().enumerate() {
                    for (kx, &kernel_val) in row.iter().enumerate() {
                        let ix = x + kx - gauss_params.radius_in_px(width as f32);
                        let iy = y + ky - gauss_params.radius_in_px(width as f32);
                        sum += kernel_val * z_original[iy * width + ix];
                    }
                }

                z_buffer[y * width + x] = sum;
            }
        }
    }

    // Write back
    for (v, &z) in vertices.iter_mut().zip(z_buffer.iter()) {
        v.position[2] = z;
    }
}
