use std::error::Error;
use std::fs;

use crate::config_file;

pub fn get(config: &config_file::ConfigFileV1) -> Result<(), Box<dyn Error>> {
    match config.remote.type_.as_str() {
        "HTTP" => {
            println!("Remote Type: HTTP");
            for file in &config.files {
                println!("File: {:?}", file);
                let file_url = format!("{}{}", config.remote.url, file.key);
                println!("File URL: {}", file_url);

                let mut resp = reqwest::blocking::get(&file_url)?;
                // TODO: check status code
                println!("Status: {}", resp.status());
                let mut file_content = Vec::new();
                resp.copy_to(&mut file_content)?;

                let file_path = format!("{}{}", file.directory, file.filename);
                println!("File Path: {}", file_path);
                fs::write(file_path, file_content)?;
            }
        },
        _ => {
            println!("Remote Type Not Implemented: {:?}", config.remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }
    Ok(())
}