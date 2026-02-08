use crate::prelude::BBox;

pub mod geonorge;

#[async_trait::async_trait]
pub trait DataProvider {
    type Output;
    async fn fetch(&self, bbox: &BBox) -> Result<Self::Output, String>;
}
