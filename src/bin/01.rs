use tokio::{fs::File, io::AsyncWriteExt};

// Coordinates
const WIDTH: f64 = 500.0;
const HEIGHT: f64 = 500.0;

const RESOLUTION: f64 = 5.0;
const COORD_SYS: usize = 5110;

const XC: f64 = 118000.0;
const YC: f64 = 1215600.0;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Assuming ems_terrain::generate is returning a Result<Vec<u8>, reqwest::Error>
    let buffer = ems_terrain::generate(XC, YC, WIDTH, HEIGHT, RESOLUTION, COORD_SYS).await?;

    // Open the file asynchronously
    let mut outfile = File::create("output\\yo.ifc").await?;

    // Write the in-memory data (Vec<u8>) to the file asynchronously
    outfile.write_all(&buffer).await?;

    Ok(())
}
