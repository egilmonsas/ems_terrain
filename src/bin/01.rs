use ems_terrain::BBox;
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bbox = BBox {
        x1: 118000.0,
        y1: 1215600.0,
        x2: 118000.0 + 100.0,
        y2: 1215600.0 + 100.0,
    };
    // Assuming ems_terrain::generate is returning a Result<Vec<u8>, reqwest::Error>
    let buffer = ems_terrain::generate(bbox, 5.0, 5110).await?;

    // Open the file asynchronously
    let mut outfile = File::create("output\\yo.ifc").await?;

    // Write the in-memory data (Vec<u8>) to the file asynchronously
    outfile.write_all(&buffer).await?;

    Ok(())
}
