use std::error::Error;
use std::fs;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigFileV1 {
    pub metadata_remote: Option<Remote>,
    pub file_remote: Remote,
    pub source_files: Vec<TargetFile>,
    pub target_files: Vec<TargetFile>,
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

pub fn read_config_file(filepath: &str) -> Result<ConfigFileV1, Box<dyn Error>> {
    // the file is .toml format, so need to read it as a string first
    let file_content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(e) => {
            let mut e_msg = String::from("Could not read file: ");
            e_msg.push_str(&filepath.to_string());
            e_msg.push_str(" ");
            e_msg.push_str(&e.to_string());
            eprintln!("{}", e_msg);
            Err(e_msg)?
        }
    };
    let parsed_config_file = match toml::from_str::<ConfigFileV1>(&file_content) {
        Ok(p) => p,
        Err(e) => {
            let mut e_msg = String::from("Could not parse file: ");
            e_msg.push_str(&filepath.to_string());
            e_msg.push_str(" ");
            e_msg.push_str(&e.to_string());
            eprintln!("{}", e_msg);
            Err(e_msg)?
        }
    };
    Ok(parsed_config_file)
}