mod route;
mod helper;
mod config_file;
mod cli_commands;

use std::env;
use std::process::exit;

use clap::Parser; // for get the ClapCli::parse() working

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
        MySubCommands::Version => {},
        MySubCommands::Get => {
            info!("getting");
            let config = match config_file::read_config_file(DEFAULT_CONFIG_FILE) {
                Ok(c) => c,
                Err(e) => {
                    error!("Could not read config file: {}", e.to_string());
                    exit(1);
                }
            };
            route::get::get(&config).expect("failed to get");
        },
        MySubCommands::Push => {
            info!("publishing");
            let config = match config_file::read_config_file(DEFAULT_CONFIG_FILE) {
                Ok(c) => c,
                Err(e) => {
                    error!("Could not read config file: {}", e.to_string());
                    exit(1);
                }
            };
            let new_config = match route::publish::publish(&config) {
                Ok(c) => c,
                Err(e) => {
                    error!("Could not publish: {}", e.to_string());
                    exit(1);
                }
            };
            info!("updating config file");
            let new_config_string = toml::to_string(&new_config).expect("failed to serialize config");
            std::fs::write(DEFAULT_CONFIG_FILE, new_config_string).expect("failed to write config");
            route::the_config_file::upload(&new_config).expect("failed to update the new config file");
        },
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
                },
                ConfigSubcommands::Download => {
                    info!("downloading config");
                    let config = match config_file::read_config_file(DEFAULT_CONFIG_FILE) {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Could not read config file: {}", e.to_string());
                            exit(1);
                        }
                    };
                    match route::the_config_file::download(&config) {
                        Ok(..) => {},
                        Err(e) => {
                            error!("Could not download config file: {}", e.to_string());
                        }
                    };
                },
                ConfigSubcommands::Upload => {
                    info!("uploading config");
                    let config = match config_file::read_config_file(DEFAULT_CONFIG_FILE) {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Could not read config file: {}", e.to_string());
                            exit(1);
                        }
                    };
                    match route::the_config_file::upload(&config) {
                        Ok(..) => {},
                        Err(e) => {
                            error!("Could not upload config file: {}", e.to_string());
                        }
                    };
                },
                ConfigSubcommands::Load { config_file_base64 } => {
                    info!("loading config");
                    match route::the_config_file::write_config_file_from_base64(DEFAULT_CONFIG_FILE, &config_file_base64) {
                        Ok(..) => {},
                        Err(e) => {
                            error!("Could not load config file: {}", e.to_string());
                        }
                    };
                },
                ConfigSubcommands::Share => {
                    info!("sharing config file");
                    match route::the_config_file::share_config_file(DEFAULT_CONFIG_FILE) {
                        Ok(..) => {},
                        Err(e) => {
                            error!("Could not share config file: {}", e.to_string());
                        }
                    };
                },
            }
        },
        MySubCommands::Dev { subcommand } => {
            match subcommand {
                DevSubcommands::GenerateExample => {
                    info!("Generating {}", EXAMPLE_CONFIG_FILE);
                    route::init::init_config_file(EXAMPLE_CONFIG_FILE, true, false).expect(&*format!("failed to init {}", EXAMPLE_CONFIG_FILE));
                },
            }
        },
        MySubCommands::Meta { subcommand } => {
            match subcommand {
                MetaSubcommands::Update => {
                    info!("updating");
                    route::update::self_update().expect("failed to self-update");
                },
            }
        },
    }
}
