mod config_file;
mod route;

use std::process::exit;

use clap::{Parser};

use crate::config_file::{read_config_file};

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
            println!("Not Yet Implemented: publish");
            let config = read_config_file("dplyt.toml").expect("failed to read the file");
            let new_config = route::publish::publish(&config).expect("failed to publish");
            let new_config_string = toml::to_string(&new_config).expect("failed to serialize config");
            std::fs::write("dplyt.toml", new_config_string).expect("failed to write config");

            exit(0);
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
