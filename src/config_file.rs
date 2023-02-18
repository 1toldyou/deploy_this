use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigFileV1 {
    pub remote: Remote,
    pub files: Vec<File>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Remote {
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub username: String,
    pub password: String,
    pub access_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    pub key: String,
    pub filename: String,
    pub directory: String,
    pub version: String,
}
