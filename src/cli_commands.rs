use clap::{Parser, Command, Subcommand};

#[derive(Parser)]
pub struct ClapCli {
    #[clap(short, long)]
    pub(crate) version: bool,

    #[clap(short, long)]
    pub(crate) debug: bool,

    #[clap(subcommand)]
    pub mode: MySubCommands,
}

#[derive(Subcommand)]
pub enum MySubCommands {
    /// Download all TargetFile(s) from the file_remote
    Get,

    /// Upload SourceFile(s) to the file_remote
    Push,

    /// Config-File related: init, download, upload, load, share
    Config {
        #[clap(subcommand)]
        subcommand: ConfigSubcommands,
    },

    /// Dev-Mode related: generate-example
    Dev {
        #[clap(subcommand)]
        subcommand: DevSubcommands,
    },

    /// Self
    Meta {
        #[clap(subcommand)]
        subcommand: MetaSubcommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigSubcommands {
    /// Create a new example config file (dplyt.toml)
    Init {
        #[clap(long)]
        overwrite: bool,
    },

    /// Download the config file (dplyt.toml) from the metadata_remote
    Download,

    /// Upload the config file (dplyt.toml) to the metadata_remote
    Upload,

    /// Load a config file from a base64 encoded string
    Load {
        #[arg()]
        config_file_base64: String,
    },

    /// Share the config file as a base64 encoded string
    Share,
}

#[derive(Subcommand)]
pub enum DevSubcommands {
    /// Generate a new example config file (dplyt.toml)
    GenerateExample,
}

#[derive(Subcommand)]
pub enum MetaSubcommands {
    /// Self-Update
    Update,
}