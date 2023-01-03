use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapManifestElement {
    pub id: String,
    pub name: String,
    pub version: String,
}
