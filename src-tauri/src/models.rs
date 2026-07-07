use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub source_path: PathBuf,
    pub created_at: String,
    pub versions: Vec<ProjectVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVersion {
    pub number: u32,
    pub note: String,
    pub created_at: String,
    pub relative_path: PathBuf,
}
