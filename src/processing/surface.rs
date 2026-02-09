use crate::{
    prelude::{Mesh, Vertex},
    processing::FilterParams,
};

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

pub fn gaussian_blur_mesh(
    mesh: &Mesh,
    width: usize,
    height: usize,
    resolution: f32,
    gauss_params: &FilterParams,
) -> Mesh {
    // TODO: implement separable kernel for better performance, but this is simpler to understand and good enough for now
    let z_buffer: Vec<f32> = mesh.vertices.iter().map(|v| v.position[2]).collect();
    let blurred = gaussian_blur_zbuffer(&z_buffer, width, height, resolution, gauss_params);
    let new_vertices = mesh
        .vertices
        .iter()
        .zip(blurred.iter())
        .map(|(v, &z)| {
            let mut v2 = *v;
            v2.position[2] = z;
            v2
        })
        .collect();

    Mesh {
        indices: mesh.indices.clone(),
        vertices: new_vertices,
    }
}

pub fn gaussian_blur_zbuffer(
    z_buffer: &[f32],
    width: usize,
    height: usize,
    resolution: f32,
    gauss_params: &FilterParams,
) -> Vec<f32> {
    let radius_px = gauss_params.radius_in_px(resolution);
    let kernel = gaussian_kernel(radius_px, gauss_params.sigma);

    let mut out = z_buffer.to_vec();

    for _ in 0..gauss_params.passes {
        let original = out.clone();

        for y in radius_px..height - radius_px {
            for x in radius_px..width - radius_px {
                let mut sum = 0.0;

                for (ky, row) in kernel.iter().enumerate() {
                    for (kx, &kv) in row.iter().enumerate() {
                        let ix = x + kx - radius_px;
                        let iy = y + ky - radius_px;
                        sum += kv * original[iy * width + ix];
                    }
                }

                out[y * width + x] = sum;
            }
        }
    }

    out
}
