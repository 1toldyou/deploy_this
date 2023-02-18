mod config_file;

use std::error::Error;
use std::fs;

use crate::config_file::ConfigFileV1;

fn main() {
    println!("Hello, world!");

    let the_config = read_config_file("example.dplt.toml").expect("failed to read the file");
    println!("{:?}", the_config);
}

fn read_config_file(filepath: &str) -> Result<ConfigFileV1, Box<dyn Error>> {
    // the file is .toml format, so need to read it as a string first
    let file_content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(e) => {
            let mut e_msg = String::from("Could not read file: ");
            e_msg.push_str(&filepath.to_string());
            e_msg.push_str(" ");
            e_msg.push_str(&e.to_string());
            eprintln!("{}", e_msg);
            Err(e_msg)?
        }
    };
    let parsed_config_file = match toml::from_str::<ConfigFileV1>(&file_content) {
        Ok(p) => p,
        Err(e) => {
            let mut e_msg = String::from("Could not parse file: ");
            e_msg.push_str(&filepath.to_string());
            e_msg.push_str(" ");
            e_msg.push_str(&e.to_string());
            eprintln!("{}", e_msg);
            Err(e_msg)?
        }
    };
    Ok(parsed_config_file)
}
