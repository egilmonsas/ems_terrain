use bytemuck::{Pod, Zeroable};
use meshopt::DecodePosition;
use meshopt::{simplify, simplify_sloppy, SimplifyOptions, VertexDataAdapter};
use spade::{DelaunayTriangulation, Point2, Triangulation};
use std::mem;

#[allow(dead_code)]
pub fn delaunay_triangulation(vertices: Vec<Vertex>) -> Vec<[usize; 3]> {
    let mut t: DelaunayTriangulation<Point2<f64>> = DelaunayTriangulation::new();
    for v in &vertices {
        t.insert(Point2::new(v.position[0] as f64, v.position[1] as f64))
            .unwrap();
    }
    let faces: Vec<[usize; 3]> = t
        .inner_faces()
        .map(|f| f.vertices().map(|v| v.index()))
        .collect();
    faces
}
pub fn triangulate_grid(width: usize, height: usize) -> Vec<[u32; 3]> {
    let mut faces = Vec::with_capacity((width - 1) * (height - 1) * 2);

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let v0 = (y * width + x) as u32;
            let v1 = v0 + 1;
            let v2 = v0 + width as u32;
            let v3 = v2 + 1;

            // triangle 1
            faces.push([v0, v2, v1]);
            // triangle 2
            faces.push([v1, v2, v3]);
        }
    }

    faces
}
// Structs
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub struct Vertex {
    position: [f32; 3],
}
impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            position: [x, y, z],
        }
    }
}
impl DecodePosition for Vertex {
    fn decode_position(&self) -> [f32; 3] {
        self.position
    }
}

#[derive(Default)]
pub struct Mesh {
    pub indices: Vec<u32>,
    pub vertices: Vec<Vertex>,
}
impl std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mesh {{ vertices: {}, indices: {} }}",
            self.vertices.len(),
            self.indices.len()
        )
    }
}

impl Mesh {
    pub fn new(indices: Vec<u32>, vertices: Vec<Vertex>) -> Self {
        Mesh { indices, vertices }
    }
    pub fn compact(&self) -> Mesh {
        // 1. Generate remap table (only vertices referenced by indices survive)
        let (new_vertex_count, remap) =
            meshopt::generate_vertex_remap(&self.vertices, Some(&self.indices));

        // 2. Remap index buffer
        let new_indices =
            meshopt::remap_index_buffer(Some(&self.indices), self.vertices.len(), &remap);

        // 3. Remap vertex buffer
        let new_vertices = meshopt::remap_vertex_buffer(&self.vertices, new_vertex_count, &remap);

        Mesh {
            vertices: new_vertices,
            indices: new_indices,
        }
    }
    pub fn simplify(&self, reduction_factor: f32) -> Mesh {
        let vertex_bytes = bytemuck::cast_slice(&self.vertices);
        let stride = mem::size_of::<Vertex>();
        let position_offset = 0; // x,y,z start at byte 0

        let adapter = VertexDataAdapter::new(vertex_bytes, stride, position_offset).unwrap();
        let mut error = 0.0;
        let options = SimplifyOptions::None;

        let simplified_indices = simplify(
            &self.indices,
            &adapter,
            (self.indices.len() as f32 * reduction_factor) as usize, // 50% reduction example
            5.0,                                                     // terrain-friendly error
            options,
            Some(&mut error),
        );

        Mesh {
            vertices: self.vertices.clone(),
            indices: simplified_indices,
        }
    }
    #[allow(dead_code)]
    pub fn simplify_sloppy(&self, reduction_factor: f32) -> Mesh {
        let vertex_bytes = bytemuck::cast_slice(&self.vertices);
        let stride = mem::size_of::<Vertex>();
        let position_offset = 0; // x,y,z start at byte 0

        let adapter = VertexDataAdapter::new(vertex_bytes, stride, position_offset).unwrap();
        let mut error = 0.0;

        let simplified_indices = simplify_sloppy(
            &self.indices,
            &adapter,
            (self.indices.len() as f32 * reduction_factor) as usize, // 50% reduction example
            5.0,                                                     // terrain-friendly error
            Some(&mut error),
        );
        Mesh {
            vertices: self.vertices.clone(),
            indices: simplified_indices,
        }
    }

    pub fn write_index_list(&self) -> String {
        let formatted_numbers: String = self
            .indices
            .chunks(3)
            .map(|chunk| format!("({},{},{})", chunk[0] + 1, chunk[1] + 1, chunk[2] + 1))
            .collect::<Vec<String>>()
            .join(",");
        format!(
            "
#53=IFCTRIANGULATEDFACESET(#94,$,$,({formatted_numbers}),$);"
        )
    }

    pub fn write_vertex_list(&self) -> String {
        let formatted_numbers: String = self
            .vertices
            .iter()
            .map(|x| {
                format!(
                    "({:.2},{:.2},{:.2})",
                    x.position[0], x.position[1], x.position[2]
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        format!(
            "
#94=IFCCARTESIANPOINTLIST3D(({formatted_numbers}));"
        )
    }
}
