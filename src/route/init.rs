use std::error::Error;
use std::fs;

use crate::config_file;

pub fn init_config_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let example_config = config_file::ConfigFileV1 {
        metadata_remote: None,
        file_remote: config_file::Remote {
            type_: String::from("HTTP"),
            url: String::from("https://gist.githubusercontent.com/1toldyou/44b6ee75f46da98259ddd258d3a6028c/raw/"),
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
        target_files: vec![
            config_file::TargetFile {
                key: String::from("something.txt"),
                filename: String::from("something.txt"),
                directory: String::from("./example-output-dir/"),
            }
        ],
    };

    // println!("{:?}", example_config);

    let toml_string = toml::to_string_pretty::<config_file::ConfigFileV1>(&example_config)?;

    // println!("{:?}", toml_string);

    // write the string to the file
    fs::write(filename, toml_string)?;

    Ok(())
}