use std::error::Error;

use awscreds::Credentials;

use crate::config_file;


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

            bucket.put_object(DEFAULT_CONFIG_FILE, config_file_string.as_bytes())?;
        },
        _ => {
            println!("Remote Type Not Implemented: {:?}", config.metadata_remote.type_);
            Err("Remote Type Not Implemented")?
        }
    }

    Ok(())
}