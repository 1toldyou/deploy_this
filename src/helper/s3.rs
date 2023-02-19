use std::error::Error;
use std::fs;

use crate::config_file;

pub fn upload_files(remote_config: &config_file::Remote, files: &Vec<config_file::SourceFile>) -> Result<Vec<config_file::TargetFile>, Box<dyn Error>> {
    if !remote_config.type_.eq("S3") {
        Err("This function only supports S3")?;
    }

    let bucket = s3::Bucket::new(
        &remote_config.bucket_name,
        s3::Region::Custom {
            region: remote_config.bucket_region.to_string(),
            endpoint: remote_config.url.to_string(),
        },
        awscreds::Credentials::new(
            Some(remote_config.access_key.as_str()),
            Some(remote_config.secret_key.as_str()),
            None,
            None,
            None
        )?,
    ).expect("failed to create bucket");

    debug!("Bucket: {:?}", bucket);

    let mut uploaded_files: Vec<config_file::TargetFile> = vec![];

    for file in files {
        info!("Uploading file: {:?}", file);
        let file_content = match fs::read_to_string(&file.local_path) {
            Ok(c) => c,
            Err(e) => {
                let e_msg = format!("Could not read file: {}: {}", file.local_path, e.to_string());
                error!("{}", e_msg);
                Err(e_msg)?
            }
        };

        let new_key = format!("{}{}", remote_config.base_dir, file.key);
        debug!("Remote Path: {}", new_key);
        bucket.put_object(new_key, file_content.as_bytes())?;

        uploaded_files.push(config_file::TargetFile {
            key: file.key.clone(),
            filename: file.target_filename.clone(),
            directory: file.target_directory.clone(),
        });
    }

    Ok(uploaded_files)
}

pub fn download_files(remote_config: &config_file::Remote, files: &Vec<config_file::TargetFile>) -> Result<(), Box<dyn Error>> {
    if !remote_config.type_.eq("S3") {
        Err("This function only supports S3")?;
    }

    let bucket = s3::Bucket::new(
        &remote_config.bucket_name,
        s3::Region::Custom {
            region: remote_config.bucket_region.to_string(),
            endpoint: remote_config.url.to_string(),
        },
        awscreds::Credentials::new(
            Some(remote_config.access_key.as_str()),
            Some(remote_config.secret_key.as_str()),
            None,
            None,
            None
        )?,
    ).expect("failed to create bucket");

    debug!("Bucket: {:?}", bucket);

    for file in files {
        debug!("File: {:?}", file);
        let new_key = format!("{}{}", remote_config.base_dir, file.key);
        debug!("Remote Path: {}", new_key);
        let object_response: s3::request_trait::ResponseData = bucket.get_object(&new_key)?;

        debug!("Status: {}", object_response.status_code());

        if !file.directory.is_empty() {
            debug!("Creating Directory: {}", file.directory);
            fs::create_dir_all(&file.directory)?;
        }

        let file_path = format!("{}{}", file.directory, file.filename);
        info!("File Path: {}", file_path);
        fs::write(file_path, object_response.bytes().to_vec())?;
    }

    Ok(())
}