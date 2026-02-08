pub mod ifc;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectMetadata {
    pub author: String,
    pub application_name: String,
    pub application_version: String,
    pub project_name: String,
    pub site_name: String,
    pub site_description: String,
}
impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            author: "Default Author".to_string(),
            application_name: "Default Application".to_string(),
            application_version: "1.0.0".to_string(),
            project_name: "Default Project".to_string(),
            site_description: "Generated terrain model".to_string(),
            site_name: "Site".to_string(),
        }
    }
}
