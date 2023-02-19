use std::error::Error;
use std::fs;

use awscreds::Credentials;

use crate::config_file;

pub fn update(config: &config_file::ConfigFileV1) -> Result<(), Box<dyn Error>> {
    const DEFAULT_CONFIG_FILE: &str = "dplyt.toml";

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

            let resp: s3::request_trait::ResponseData = bucket.get_object(&DEFAULT_CONFIG_FILE)?;

            debug!("Status: {}", resp.status_code());

            fs::write(DEFAULT_CONFIG_FILE, resp.bytes().to_vec())?;
        }
        _ => {
            error!("Remote Type Not Implemented: {:?}", config.file_remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }

    Ok(())
}