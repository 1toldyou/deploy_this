use std::error::Error;
use std::fs;

use serde_derive::{Deserialize, Serialize};

use crate::helper::check_version::is_same_major_or_minor_version;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigFileV1 {
    pub edition: String,
    pub version: String,
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
    pub username: String,
    pub password: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket_name: String,
    pub bucket_region: String,
    pub require_credentials: bool,
    pub ignore_version: bool,
    pub ignore_checksum: bool,
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
    pub version: String,
    pub checksum: String,
}

const DEFAULT_CONFIG_FILE: &str = "dplyt.toml";

pub fn get_default_config_file_as_source_file() -> SourceFile {
    return SourceFile {
        key: DEFAULT_CONFIG_FILE.to_string(),
        local_path: DEFAULT_CONFIG_FILE.to_string(),
        target_filename: DEFAULT_CONFIG_FILE.to_string(),
        target_directory: "./".to_string(),
    }
}

pub fn get_default_config_file_as_target_file() -> TargetFile {
    return TargetFile {
        key: DEFAULT_CONFIG_FILE.to_string(),
        filename: DEFAULT_CONFIG_FILE.to_string(),
        directory: "./".to_string(),
        version: "0.0.0".to_string(),
        checksum: "".to_string(),
    }
}

pub fn read_config_file(filepath: &str) -> Result<ConfigFileV1, Box<dyn Error>> {
    // the file is .toml format, so need to read it as a string first
    let file_content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(e) => {
            let e_msg = format!("Could not read file: {} {}", filepath, e.to_string());
            eprintln!("{}", e_msg);
            Err(e_msg)?
        }
    };
    let parsed_config_file = match toml::from_str::<ConfigFileV1>(&file_content) {
        Ok(p) => p,
        Err(e) => {
            let e_msg = format!("Could not parse file: {} {}", filepath, e.to_string());
            eprintln!("{}", e_msg);
            Err(e_msg)?
        }
    };

    if !is_same_major_or_minor_version(&parsed_config_file.edition) {
        let e_msg = format!(
            "Config file edition does not match: `{}`, required: `^{}`",
            parsed_config_file.edition, env!("CARGO_PKG_VERSION")
        );
        error!("{}", e_msg);
        Err(e_msg)?
    }

    Ok(parsed_config_file)
}