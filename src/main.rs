mod config_file;
mod route;

use std::error::Error;
use std::fs;
use std::process::exit;

use clap::{Parser};

use crate::config_file::ConfigFileV1;

#[derive(Parser)]
struct ClapCli {
    #[arg(default_value = "init")]
    mode: String,
}

fn main() {
    println!("Deploy This");

    let clap_args = ClapCli::parse();

    match clap_args.mode.to_owned().as_str() {
        "generate-example" => {
            println!("Generating example.dplyt.toml");
            route::init::init_config_file("example.dplyt.toml").expect("failed to generate example.dplt.toml");
            exit(0);
        },
        "init" => {
            println!("Generating dplyt.toml");
            route::init::init_config_file("dplyt.toml").expect("failed to init dplt.toml");
            exit(0);
        },
        "publish" => {
            eprintln!("Not Yet Implemented: publish");
            exit(1);
        },
        "get" => {
            let config = read_config_file("dplyt.toml").expect("failed to read the file");
            route::get::get(&config).expect("failed to get");
            exit(0);
        },
        _ => {
            println!("Default Mode!");
            let the_config = read_config_file("example.dplt.toml").expect("failed to read the file");
            println!("{:?}", the_config);
            exit(0);
        }
    }
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
