use clap::{Parser, Subcommand};
use std::process;
mod config;
use config::Config;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Config(config_args) => match &config_args.action {
            ConfigAction::Get { key } => {
                if key != "core.root" {
                    eprintln!("Error: Only 'core.root' is supported for the get command.");
                    process::exit(1);
                }

                let config = Config::load().unwrap_or_else(|err| {
                    eprintln!("Error loading configuration: {}", err);
                    process::exit(1);
                });

                match config.get_value(key) {
                    Some(value) => {
                        println!("{}", value.as_str().unwrap_or_default());
                    }
                    None => {
                        // This case should ideally not be reached for core.root due to default handling in Config::load
                        eprintln!("Error: Could not find value for key '{}'", key);
                        process::exit(1);
                    }
                }
            }
        },
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Work with ghwt configuration
    Config(ConfigArgs),
}

#[derive(clap::Args, Debug)]
struct ConfigArgs {
    #[command(subcommand)]
    action: ConfigAction,
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Get a configuration value
    Get {
        /// The key to get
        #[arg(value_name = "KEY")]
        key: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_get_with_key() {
        let cli = Cli::try_parse_from(&["ghwt", "config", "get", "core.root"]).unwrap();
        match cli.command {
            Commands::Config(config_args) => match config_args.action {
                ConfigAction::Get { key } => {
                    assert_eq!(key, "core.root");
                }
            },
        }
    }

    #[test]
    fn test_parse_config_get_missing_key() {
        let err = Cli::try_parse_from(&["ghwt", "config", "get"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }
}
