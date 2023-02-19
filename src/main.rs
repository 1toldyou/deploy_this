mod route;
mod config_file;
mod helper;

use std::process::exit;

use clap::{Parser};

#[derive(Parser)]
struct ClapCli {
    #[arg(default_value = "init")]
    mode: String,
}

fn main() {
    println!("Deploy This v{}", env!("CARGO_PKG_VERSION"));

    let clap_args = ClapCli::parse();

    const EXAMPLE_CONFIG_FILE: &str = "example.dplyt.toml";
    const DEFAULT_CONFIG_FILE: &str = "dplyt.toml";

    match clap_args.mode.to_owned().as_str() {
        "generate-example" => {
            println!("Generating {}", EXAMPLE_CONFIG_FILE);
            route::init::init_config_file(EXAMPLE_CONFIG_FILE).expect(&*format!("failed to init {}", EXAMPLE_CONFIG_FILE));
            exit(0);
        },
        "init" => {
            println!("Generating {}", DEFAULT_CONFIG_FILE);
            route::init::init_config_file(DEFAULT_CONFIG_FILE).expect(&*format!("failed to init {}", DEFAULT_CONFIG_FILE));
            exit(0);
        },
        "publish" => {
            println!("publishing");
            let config = helper::read_config_file(DEFAULT_CONFIG_FILE).expect("failed to read the file");
            let new_config = route::publish::publish(&config).expect("failed to publish");
            let new_config_string = toml::to_string(&new_config).expect("failed to serialize config");
            std::fs::write(DEFAULT_CONFIG_FILE, new_config_string).expect("failed to write config");
            helper::upload_config_file(&new_config).expect("failed to upload config");
            exit(0);
        },
        "get" => {
            let config = helper::read_config_file(DEFAULT_CONFIG_FILE).expect("failed to read the file");
            route::get::get(&config).expect("failed to get");
            exit(0);
        },
        "update" => {
            let config = helper::read_config_file(DEFAULT_CONFIG_FILE).expect("failed to read the file");
            route::update::update(&config).expect("failed to update");
            exit(0);
        },
        _ => {
            println!("Default Mode!");
            let the_config = helper::read_config_file(EXAMPLE_CONFIG_FILE).expect("failed to read the file");
            println!("{:?}", the_config);
            exit(0);
        }
    }
}
