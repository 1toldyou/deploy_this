use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigFileV1 {
    pub metadata_remote: Remote,
    pub file_remote: Remote,
    pub source_files: Vec<SourceFile>,
    pub target_files: Vec<TargetFile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Remote {
    #[serde(rename = "type")]
    pub type_: String,
    pub base_dir: String,
    pub url: String,
    pub require_credentials: bool,
    pub username: String,
    pub password: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket_name: String,
    pub bucket_region: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceFile {
    pub key: String,
    pub local_path: String,
    pub target_filename: String,
    pub target_directory: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TargetFile {
    pub key: String,
    pub filename: String,
    pub directory: String,
}