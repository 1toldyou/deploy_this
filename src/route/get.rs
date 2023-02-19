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
        _ => {
            println!("Remote Type Not Implemented: {:?}", config.file_remote.type_);
            Err("Remote Type Not Implemented")?;
        }
    }
    Ok(())
}