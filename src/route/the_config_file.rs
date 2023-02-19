use std::error::Error;

use crate::config_file;
use crate::helper;

pub fn upload(config: &config_file::ConfigFileV1) -> Result<(), Box<dyn Error>> {
    match config.file_remote.type_.as_str() {
        "S3" => {
            info!("Metadata Remote Type: S3");
            helper::s3::upload_files(&config.metadata_remote, &vec![config_file::get_default_config_file_as_source_file()])?;
        }
        _ => {
            error!("Remote Type Not Implemented: {:?}", config.file_remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }

    Ok(())
}

pub fn download(config: &config_file::ConfigFileV1) -> Result<(), Box<dyn Error>> {
    match config.file_remote.type_.as_str() {
        "S3" => {
            info!("Metadata Remote Type: S3");
            helper::s3::download_files(&config.metadata_remote, &vec![config_file::get_default_config_file_as_target_file()])?;
        }
        _ => {
            error!("Remote Type Not Implemented: {:?}", config.file_remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }

    Ok(())
}