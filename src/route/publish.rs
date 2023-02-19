use std::error::Error;
use std::fs;

use awscreds::Credentials;

use crate::config_file;

pub fn publish(config: &config_file::ConfigFileV1) -> Result<(), Box<dyn Error>> {
    match config.file_remote.type_.as_str() {
        "S3" => {
            println!("Remote Type: S3");
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

            println!("Bucket: {:?}", bucket);

            for file in &config.source_files {
                println!("Uploading file: {:?}", file);
                let file_content = match fs::read_to_string(&file.local_path) {
                    Ok(c) => c,
                    Err(e) => {
                        let mut e_msg = String::from("Could not read file: ");
                        e_msg.push_str(&file.local_path.to_string());
                        e_msg.push_str(" ");
                        e_msg.push_str(&e.to_string());
                        eprintln!("{}", e_msg);
                        Err(e_msg)?
                    }
                };
                let key = format!("{}/{}", file.target_directory, file.target_filename);
                bucket.put_object(&key, file_content.as_bytes())?;
            }
        }
        _ => {
            println!("Remote Type Not Implemented: {:?}", config.file_remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }
    Ok(())
}