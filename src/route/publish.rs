use std::error::Error;

use crate::config_file;

pub fn publish(config: &config_file::ConfigFileV1) -> Result<config_file::ConfigFileV1, Box<dyn Error>> {
    let mut uploaded_files: Vec<config_file::TargetFile> = vec![];

    match config.file_remote.type_.as_str() {
        "S3" => {
            info!("Remote Type: S3");
            uploaded_files = crate::helper::s3::upload_files(&config.file_remote, &config.source_files)?;
        }
        _ => {
            error!("Remote Type Not Implemented: {:?}", config.file_remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }

    Ok(config_file::ConfigFileV1 {
        edition: config.edition.clone(),
        metadata_remote: config.metadata_remote.clone(),
        file_remote: config.file_remote.clone(),
        source_files: config.source_files.clone(),
        target_files: uploaded_files,
    })
}