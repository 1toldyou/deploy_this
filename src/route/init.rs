use std::error::Error;
use std::fs;

use crate::config_file;

pub fn init_config_file(filename: &str, overwrite: bool, ask_question: bool) -> Result<(), Box<dyn Error>> {
    let mut example_config = config_file::ConfigFileV1 {
        name: String::from(""),
        edition: env!("CARGO_PKG_VERSION").to_string(),
        version: String::from("0.0.0"),
        metadata_remote: config_file::Remote {
            type_: String::from(""),
            base_dir: String::from(""),
            url: String::from(""),
            username: String::from(""),
            password: String::from(""),
            access_key: String::from(""),
            secret_key: String::from(""),
            bucket_name: String::from(""),
            bucket_region: String::from(""),
            require_credentials: true,
            ignore_version: false,
            ignore_checksum: false,
        },
        file_remote: config_file::Remote {
            type_: String::from(""),
            base_dir: String::from(""),
            url: String::from(""),
            username: String::from(""),
            password: String::from(""),
            access_key: String::from(""),
            secret_key: String::from(""),
            bucket_name: String::from(""),
            bucket_region: String::from(""),
            require_credentials: true,
            ignore_version: true,
            ignore_checksum: true,
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

    if ask_question {
        println!("Fill out for the config file, you can leave it empty and change it later.");
        example_config.name = dialoguer::Input::<String>::new().with_prompt("Name").interact()?;
        let remote_types = vec!["S3", "FTP"];
        let remote_type_index = dialoguer::Select::new().with_prompt("Remote Type").items(&remote_types).interact()?;
        let remote_type = remote_types[remote_type_index].to_string();
        example_config.metadata_remote.type_ = remote_type.clone();
        example_config.file_remote.type_ = remote_type.clone();
        match remote_type.as_str() {
            "S3" => {
                example_config.metadata_remote.bucket_name = dialoguer::Input::<String>::new().with_prompt("Bucket Name").interact()?;
                example_config.file_remote.bucket_name = example_config.metadata_remote.bucket_name.clone();
            },
            _ => {
                info!("Not implemented yet: {}", remote_type);
            }
        }
    }

    let toml_string = toml::to_string_pretty::<config_file::ConfigFileV1>(&example_config)?;


    if fs::metadata(filename).is_ok() {
        if overwrite {
            info!("Overwriting existing config file: {}", filename);
            fs::write(filename, toml_string)?;
            Ok(())
        }
        else {
            info!("Config file already exists: {}", filename);
            Err("Config file already exists")?
        }
    }
    else {
        info!("Writing config file: {}", filename);
        fs::write(filename, toml_string)?;
        Ok(())
    }
}