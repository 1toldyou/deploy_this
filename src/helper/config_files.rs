use std::error::Error;
use std::fs;

use awscreds::Credentials;

use crate::config_file;

pub fn read_config_file(filepath: &str) -> Result<config_file::ConfigFileV1, Box<dyn Error>> {
    // the file is .toml format, so need to read it as a string first
    let file_content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(e) => {
            let e_msg = format!("Could not read file: {} {}", filepath, e.to_string());
            eprintln!("{}", e_msg);
            Err(e_msg)?
        }
    };
    let parsed_config_file = match toml::from_str::<config_file::ConfigFileV1>(&file_content) {
        Ok(p) => p,
        Err(e) => {
            let e_msg = format!("Could not parse file: {} {}", filepath, e.to_string());
            eprintln!("{}", e_msg);
            Err(e_msg)?
        }
    };
    Ok(parsed_config_file)
}

pub fn upload_config_file(config: &config_file::ConfigFileV1) -> Result<(), Box<dyn Error>> {
    const DEFAULT_CONFIG_FILE: &str = "dplyt.toml";
    let config_file_string = toml::to_string_pretty::<config_file::ConfigFileV1>(&config)?;

    match config.metadata_remote.type_.as_str() {
        "S3" => {
            let bucket = s3::Bucket::new(
                &config.metadata_remote.bucket_name,
                s3::Region::Custom {
                    region: config.metadata_remote.bucket_region.to_string(),
                    endpoint: config.metadata_remote.url.to_string(),
                },
                Credentials::new(
                    Some(config.metadata_remote.access_key.as_str()),
                    Some(config.metadata_remote.secret_key.as_str()),
                    None,
                    None,
                    None
                )?,
            ).expect("failed to create bucket");

            let new_key = format!("{}{}", config.file_remote.base_dir, DEFAULT_CONFIG_FILE);
            debug!("Remote Path: {}", new_key);
            bucket.put_object(new_key, config_file_string.as_bytes())?;
        },
        _ => {
            println!("Remote Type Not Implemented: {:?}", config.metadata_remote.type_);
            Err("Remote Type Not Implemented")?
        }
    }

    Ok(())
}