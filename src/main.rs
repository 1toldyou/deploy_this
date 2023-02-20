mod route;
mod config_file;
mod helper;
mod cli_commands;

use std::env;
use std::process::exit;
use clap::Parser;

#[macro_use]
extern crate log;

use crate::cli_commands::{ClapCli, MySubCommands, ConfigSubcommands, DevSubcommands, MetaSubcommands};
use crate::config_file::DEFAULT_CONFIG_FILE;

const EXAMPLE_CONFIG_FILE: &str = "example.dplyt.toml";

fn main() {
    let clap_args: ClapCli = ClapCli::parse();

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

    match clap_args.mode {
        MySubCommands::Get => {
            info!("getting");
            let config = match config_file::read_config_file(DEFAULT_CONFIG_FILE) {
                Ok(c) => c,
                Err(e) => {
                    error!("Could not read config file: {}", e);
                }
            };
            route::get::get(&config).expect("failed to get");
        }
        MySubCommands::Push => {
            info!("publishing");
            let config = match config_file::read_config_file(DEFAULT_CONFIG_FILE) {
                Ok(c) => c,
                Err(e) => {
                    error!("Could not read config file: {}", e.to_string());
                }
            };
            route::publish::publish(&config).expect("failed to push");
        }
        MySubCommands::Config { subcommand } => {
            match subcommand {
                ConfigSubcommands::Init { overwrite } => {
                    info!("Generating {}", DEFAULT_CONFIG_FILE);
                    match route::init::init_config_file(DEFAULT_CONFIG_FILE, overwrite, true) {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Could not init config file: {}", e.to_string());
                        }
                    };
                }
                ConfigSubcommands::Download => {
                    info!("downloading config");
                    let config = match config_file::read_config_file(DEFAULT_CONFIG_FILE) {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Could not read config file: {}", e.to_string());
                        }
                    };
                    match route::the_config_file::download(&config) {
                        Ok(..) => {},
                        Err(e) => {
                            error!("Could not download config file: {}", e.to_string());
                        }
                    };
                }
                ConfigSubcommands::Upload => {
                    info!("uploading config");
                    let config = match config_file::read_config_file(DEFAULT_CONFIG_FILE) {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Could not read config file: {}", e.to_string());
                        }
                    };
                    match route::the_config_file::upload(&config) {
                        Ok(..) => {},
                        Err(e) => {
                            error!("Could not upload config file: {}", e.to_string());
                        }
                    };
                }
                ConfigSubcommands::Load { .. } => {}
                ConfigSubcommands::Share => {
                    info!("sharing config file");
                    match route::the_config_file::share_config_file(DEFAULT_CONFIG_FILE) {
                        Ok(..) => {},
                        Err(e) => {
                            error!("Could not share config file: {}", e.to_string());
                        }
                    };
                }
            }
        }
        MySubCommands::Dev { subcommand } => {
            match subcommand {
                DevSubcommands::GenerateExample => {
                    info!("Generating {}", EXAMPLE_CONFIG_FILE);
                    route::init::init_config_file(EXAMPLE_CONFIG_FILE, true, false).expect(&*format!("failed to init {}", EXAMPLE_CONFIG_FILE));
                }
            }
        }
        MySubCommands::Meta { subcommand } => {
            match subcommand {
                MetaSubcommands::Update => {
                    info!("updating");
                    route::update::self_update().expect("failed to self-update");
                }
            }
        }
    }
}
