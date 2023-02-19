use std::error::Error;
use std::fs;

use crate::config_file;

pub fn init_config_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let example_config = config_file::ConfigFileV1 {
        metadata_remote: None,
        file_remote: config_file::FileRemote {
            type_: String::from("HTTP"),
            url: String::from("https://gist.githubusercontent.com/1toldyou/44b6ee75f46da98259ddd258d3a6028c/raw"),
            username: String::from(""),
            password: String::from(""),
            access_key: String::from(""),
        },
        target_files: vec![
            config_file::File{
                key: String::from("/something.txt"),
                filename: String::from("something.txt"),
                directory: String::from("./example-output-dir/"),
                version: String::from("2023.2.18"),
            }
        ],
        source_files: vec![],
    };

    // println!("{:?}", example_config);

    let toml_string = toml::to_string_pretty::<config_file::ConfigFileV1>(&example_config)?;

    // println!("{:?}", toml_string);

    // write the string to the file
    fs::write(filename, toml_string)?;

    Ok(())
}