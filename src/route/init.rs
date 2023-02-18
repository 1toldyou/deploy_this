use std::error::Error;
use std::fs;

use crate::config_file;

pub fn init_config_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let example_config = config_file::ConfigFileV1 {
        remote: config_file::Remote {
            type_: String::from("FTP"),
            url: String::from(""),
            username: String::from(""),
            password: String::from(""),
            access_key: String::from(""),
        },
        files: vec![
            config_file::File{
                filename: String::from("1.txt"),
                directory: String::from("./")
            }
        ],
    };

    println!("{:?}", example_config);

    let toml_string = toml::to_string_pretty::<config_file::ConfigFileV1>(&example_config)?;

    println!("{:?}", toml_string);

    // write the string to the file


    fs::write(filename, toml_string)?;

    Ok(())
}