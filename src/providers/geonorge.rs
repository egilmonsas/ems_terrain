use crate::{prelude::BBox, providers::DataProvider};
use reqwest::Client;

// providers/geonorge.rs
pub struct TerrainProvider {
    pub resolution: f32,
    pub crs: usize,
}

impl TerrainProvider {
    #[must_use]
    pub fn new(resolution: f32, crs: usize) -> Self {
        TerrainProvider { resolution, crs }
    }
}

#[async_trait::async_trait]
impl DataProvider for TerrainProvider {
    type Output = Vec<u8>;
    async fn fetch(&self, bbox: &BBox) -> Result<Self::Output, String> {
        let padding: f32 = 5.0 * self.resolution;
        let num_pixels_x = bbox.num_pixels_x(self.resolution);
        let num_pixels_y = bbox.num_pixels_y(self.resolution);
        let url = format!(
            "http://wcs.geonorge.no/skwms1/wcs.hoyde-dtm_somlos?SERVICE=WCS&VERSION=1.0.0&REQUEST=GetCoverage&COVERAGE=las_dtm&CRS=EPSG:{}&BBOX={},{},{},{}&WIDTH={}&HEIGHT={}&FORMAT=GeoTIFF",
            self.crs,bbox.x1-padding, bbox.y1-padding, bbox.x2+padding, bbox.y2+padding,  num_pixels_x, num_pixels_y
        );
        println!("Fetching terrain data from GeoNorge with URL: {}", &url);
        let client = Client::new();
        let response = client.get(&url).send().await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
                    Ok(bytes.to_vec())
                } else {
                    Err(format!("Failed to fetch data: HTTP {}", response.status()))
                }
            }
            Err(e) => Err(format!("Request error: {}", e)),
        }
    }
}
