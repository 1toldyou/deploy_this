use std::error::Error;
use std::fs;

use crate::config_file;

pub fn init_config_file(filename: &str, overwrite: bool) -> Result<(), Box<dyn Error>> {
    let example_config = config_file::ConfigFileV1 {
        edition: env!("CARGO_PKG_VERSION").to_string(),
        metadata_remote: config_file::Remote {
            type_: String::from("S3"),
            base_dir: String::from(""),
            url: String::from("https://gateway.storjshare.io"),
            require_credentials: true,
            username: String::from(""),
            password: String::from(""),
            access_key: String::from(""),
            secret_key: String::from(""),
            bucket_name: String::from(""),
            bucket_region: String::from(""),
        },
        file_remote: config_file::Remote {
            type_: String::from("S3"),
            base_dir: String::from(""),
            url: String::from("https://gateway.storjshare.io"),
            require_credentials: false,
            username: String::from(""),
            password: String::from(""),
            access_key: String::from(""),
            secret_key: String::from(""),
            bucket_name: String::from(""),
            bucket_region: String::from(""),
        },
        source_files: vec![
            config_file::SourceFile {
                key: String::from("readme"),
                local_path: String::from("./README.md"),
                target_filename: String::from("README.md"),
                target_directory: String::from("./"),
            }
        ],
        target_files: vec![],
    };

    // println!("{:?}", example_config);

    let toml_string = toml::to_string_pretty::<config_file::ConfigFileV1>(&example_config)?;

    // println!("{:?}", toml_string);

    // check is file exists
    if fs::metadata(filename).is_ok() {
        if overwrite {
            info!("Overwriting existing config file: {}", filename);
            fs::write(filename, toml_string)?;
        } else {
            info!("Config file already exists: {}", filename);
            Err("Config file already exists")?;
        }
    }

    Ok(())
}