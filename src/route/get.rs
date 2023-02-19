use std::error::Error;
use std::fs;

use crate::config_file;

pub fn get(config: &config_file::ConfigFileV1) -> Result<(), Box<dyn Error>> {
    match config.file_remote.type_.as_str() {
        "HTTP" => {
            println!("Remote Type: HTTP");
            for file in &config.target_files {
                println!("File: {:?}", file);
                let file_url = format!("{}{}", config.file_remote.url, file.key);
                println!("File URL: {}", file_url);

                let mut resp = reqwest::blocking::get(&file_url)?;
                // TODO: check status code
                println!("Status: {}", resp.status());
                let mut file_content = Vec::new();
                resp.copy_to(&mut file_content)?;

                // create the directory if it doesn't exist
                if !file.directory.is_empty() {
                    println!("Directory: {}", file.directory);
                    fs::create_dir_all(&file.directory)?;
                }

                let file_path = format!("{}{}", file.directory, file.filename);
                println!("File Path: {}", file_path);
                fs::write(file_path, file_content)?;
            }
        },
        "S3" => {
            println!("Remote Type: S3");
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

            println!("Bucket: {:?}", bucket);

            for file in &config.target_files {
                println!("File: {:?}", file);
                let object_response: s3::request_trait::ResponseData = bucket.get_object(&file.key)?;

                println!("Status: {}", object_response.status_code());

                if !file.directory.is_empty() {
                    println!("Directory: {}", file.directory);
                    fs::create_dir_all(&file.directory)?;
                }

                let file_path = format!("{}{}", file.directory, file.filename);
                println!("File Path: {}", file_path);
                fs::write(file_path, object_response.bytes().to_vec())?;
            }
        },
        _ => {
            println!("Remote Type Not Implemented: {:?}", config.file_remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }
    Ok(())
}