mod route;
mod config_file;
mod helper;

use std::env;
use std::process::exit;

use clap::{Parser};
#[macro_use]
extern crate log;

#[derive(Parser)]
struct ClapCli {
    #[clap(short, long)]
    version: bool,

    #[arg(default_value = "init")]
    mode: String,

    #[clap(long)]
    debug: bool,
}

const EXAMPLE_CONFIG_FILE: &str = "example.dplyt.toml";
const DEFAULT_CONFIG_FILE: &str = "dplyt.toml";

fn main() {
    let clap_args = ClapCli::parse();

    if clap_args.version {
        println!("Deploy This v{}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    // TODO: better way to set log level
    if clap_args.debug {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("Deploy This v{}", env!("CARGO_PKG_VERSION"));

    match clap_args.mode.to_owned().as_str() {
        "generate-example" => {
            info!("Generating {}", EXAMPLE_CONFIG_FILE);
            route::init::init_config_file(EXAMPLE_CONFIG_FILE).expect(&*format!("failed to init {}", EXAMPLE_CONFIG_FILE));
            exit(0);
        },
        "init" => {
            info!("Generating {}", DEFAULT_CONFIG_FILE);
            route::init::init_config_file(DEFAULT_CONFIG_FILE).expect(&*format!("failed to init {}", DEFAULT_CONFIG_FILE));
            exit(0);
        },
        "publish" => {
            info!("publishing");
            let config = helper::read_config_file(DEFAULT_CONFIG_FILE).expect("failed to read the file");
            let new_config = route::publish::publish(&config).expect("failed to publish");
            let new_config_string = toml::to_string(&new_config).expect("failed to serialize config");
            std::fs::write(DEFAULT_CONFIG_FILE, new_config_string).expect("failed to write config");
            helper::upload_config_file(&new_config).expect("failed to upload config");
            exit(0);
        },
        "get" => {
            info!("getting");
            let config = helper::read_config_file(DEFAULT_CONFIG_FILE).expect("failed to read the file");
            route::get::get(&config).expect("failed to get");
            exit(0);
        },
        "update" => {
            info!("updating");
            let config = helper::read_config_file(DEFAULT_CONFIG_FILE).expect("failed to read the file");
            route::update::update(&config).expect("failed to update");
            exit(0);
        },
        _ => {
            info!("Default Mode");
            let the_config = helper::read_config_file(EXAMPLE_CONFIG_FILE).expect("failed to read the file");
            debug!("{:?}", the_config);
            exit(0);
        }
    }
}
