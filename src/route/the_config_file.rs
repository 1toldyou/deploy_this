use std::error::Error;
use std::fs;

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

pub fn share_config_file(filepath: &str) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(filepath)?;
    #[allow(deprecated)]
    let base64_string = base64::encode(&file_content);

    println!("The {} in Base64 \ndplyt --config-file-base64 {} load-config", filepath, base64_string);

    Ok(())
}

pub fn write_config_file_from_base64(filepath: &str, encoded_config_file: &str) -> Result<(), Box<dyn Error>> {
    #[allow(deprecated)]
    fs::write(filepath, base64::decode(encoded_config_file)?)?;
    Ok(())
}