use std::error::Error;
use std::fs;

use crate::config_file;

pub fn get(config: &config_file::ConfigFileV1) -> Result<(), Box<dyn Error>> {
    match config.file_remote.type_.as_str() {
        "HTTP" => {
            info!("Remote Type: HTTP");
            for file in &config.target_files {
                debug!("File: {:?}", file);
                let file_url = format!("{}{}", config.file_remote.url, file.key);
                debug!("File URL: {}", file_url);

                let mut resp = reqwest::blocking::get(&file_url)?;
                // TODO: check status code
                debug!("Status: {}", resp.status());
                let mut file_content = Vec::new();
                resp.copy_to(&mut file_content)?;

                // create the directory if it doesn't exist
                if !file.directory.is_empty() {
                    debug!("Directory: {}", file.directory);
                    fs::create_dir_all(&file.directory)?;
                }

                let file_path = format!("{}{}", file.directory, file.filename);
                info!("File Path: {}", file_path);
                fs::write(file_path, file_content)?;
            }
        },
        "S3" => {
            info!("Remote Type: S3");
            let bucket = s3::Bucket::new(
                &config.file_remote.bucket_name,
                s3::Region::Custom {
                    region: config.file_remote.bucket_region.to_string(),
                    endpoint: config.file_remote.url.to_string(),
                },
                awscreds::Credentials::new(
                    Some(config.file_remote.access_key.as_str()),
                    Some(config.file_remote.secret_key.as_str()),
                    None,
                    None,
                    None
                )?,
            ).expect("failed to create bucket");

            debug!("Bucket: {:?}", bucket);

            for file in &config.target_files {
                debug!("File: {:?}", file);
                let new_key = format!("{}{}", config.file_remote.base_dir, file.key);
                debug!("Remote Path: {}", new_key);
                let object_response: s3::request_trait::ResponseData = bucket.get_object(&new_key)?;

                debug!("Status: {}", object_response.status_code());

                if !file.directory.is_empty() {
                    debug!("Directory: {}", file.directory);
                    fs::create_dir_all(&file.directory)?;
                }

                let file_path = format!("{}{}", file.directory, file.filename);
                info!("File Path: {}", file_path);
                fs::write(file_path, object_response.bytes().to_vec())?;
            }
        },
        _ => {
            error!("Remote Type Not Implemented: {:?}", config.file_remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }
    Ok(())
}