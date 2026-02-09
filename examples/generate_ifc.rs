use ems_terrain::prelude::*;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting IFC Generation Pipeline...");

    // 1. Setup the request (Simulating UI input from Tauri)
    let request = GenerateRequest {
        resolution: 1.0,
        crs: 5110,
        ..Default::default()
    };

    println!("✅ Generated request: {request:#?}");
    let ifc_data = generate_ifc_terrain(request).await.unwrap();
    let mut file = File::create("output.ifc").unwrap();
    file.write_all(&ifc_data).unwrap();
    Ok(())
}

/// The main entry point for your Tauri backend
pub async fn generate_ifc_terrain(req: GenerateRequest) -> Result<Vec<u8>, String> {
    // 1. Fetch
    let terrain_provider = TerrainProvider::new(req.resolution, req.crs);
    let raw_data = terrain_provider
        .fetch(&req.bbox)
        .await
        .map_err(|e| format!("Failed to fetch terrain: {}", e))?;

    // 2. Create Mesh
    let mesh = Mesh::from_geotiff(&raw_data, &req.bbox, req.resolution);

    // 3. Process
    let mesh = gaussian_blur_mesh(
        &mesh,
        req.bbox.num_pixels_x(req.resolution),
        req.bbox.num_pixels_y(req.resolution),
        req.resolution,
        &req.post_process_params,
    );

    let final_mesh = mesh
        .simplify(req.post_process_params.compression_factor)
        .compact();

    // 4. Export
    let mut writer = IfcWriter::new(None);
    writer.add_mesh(&final_mesh);

    Ok(writer.finish())
}
