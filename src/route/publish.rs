use std::error::Error;
use std::fs;

use awscreds::Credentials;

use crate::config_file;

pub fn publish(config: &config_file::ConfigFileV1) -> Result<config_file::ConfigFileV1, Box<dyn Error>> {
    let mut uploaded_files: Vec<config_file::TargetFile> = vec![];

    match config.file_remote.type_.as_str() {
        "S3" => {
            info!("Remote Type: S3");
            let bucket = s3::Bucket::new(
                &config.file_remote.bucket_name,
                s3::Region::Custom {
                    region: config.file_remote.bucket_region.to_string(),
                    endpoint: config.file_remote.url.to_string(),
                },
                Credentials::new(
                    Some(config.file_remote.access_key.as_str()),
                    Some(config.file_remote.secret_key.as_str()),
                    None,
                    None,
                    None
                )?,
            ).expect("failed to create bucket");

            debug!("Bucket: {:?}", bucket);

            for file in &config.source_files {
                info!("Uploading file: {:?}", file);
                let file_content = match fs::read_to_string(&file.local_path) {
                    Ok(c) => c,
                    Err(e) => {
                        let e_msg = format!("Could not read file: {} {}", file.local_path, e.to_string());
                        error!("{}", e_msg);
                        Err(e_msg)?
                    }
                };

                let new_key = format!("{}{}", config.file_remote.base_dir, file.key);
                debug!("Key: {}", new_key);
                bucket.put_object(new_key, file_content.as_bytes())?;

                uploaded_files.push(config_file::TargetFile {
                    key: file.key.clone(),
                    filename: file.target_filename.clone(),
                    directory: file.target_directory.clone(),
                });
            }
        }
        _ => {
            error!("Remote Type Not Implemented: {:?}", config.file_remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }

    Ok(config_file::ConfigFileV1 {
        metadata_remote: config.metadata_remote.clone(),
        file_remote: config.file_remote.clone(),
        source_files: config.source_files.clone(),
        target_files: uploaded_files,
    })
}