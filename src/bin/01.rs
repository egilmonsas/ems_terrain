use ems_terrain::BBox;
use ems_terrain::PostProcessParams;
use tokio::{fs::File, io::AsyncWriteExt};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bbox = BBox {
        // CU
        x1: 112887.0 - 100.0,
        y1: 1217103.0 - 100.0,
        x2: 112887.0 + 100.0,
        y2: 1217103.0 + 100.0,
    };
    // Assuming ems_terrain::generate is returning a Result<Vec<u8>, reqwest::Error>
    let buffer = ems_terrain::generate(bbox, 1.0, PostProcessParams::default(), 5110).await?;

    // Open the file asynchronously
    let mut outfile = File::create("output\\yo.ifc").await?;

    // Write the in-memory data (Vec<u8>) to the file asynchronously
    outfile.write_all(&buffer).await?;

    Ok(())
}
