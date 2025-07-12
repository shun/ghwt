use clap::{Parser, Subcommand};
use std::process;
mod config;
use config::Config;
mod commands;
pub use commands::config_set::SetArgs;

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
                    Some(value) => match value.as_str() {
                        Some(s) => println!("{}", s),
                        None => {
                            eprintln!("Error: Configuration value for '{}' is not a string", key);
                            process::exit(1);
                        }
                    },
                    None => {
                        // This case should ideally not be reached for core.root due to default handling in Config::load
                        eprintln!("Error: Could not find value for key '{}'", key);
                        process::exit(1);
                    }
                }
            }
            ConfigAction::Set(args) => {
                if let Err(e) = commands::config_set::handle_config_set(args.clone()) {
                    eprintln!("Error: {}", e);
                    process::exit(1);
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
    /// Set a configuration value
    Set(SetArgs),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_get_with_key() {
        let cli = Cli::try_parse_from(["ghwt", "config", "get", "core.root"]).unwrap();
        match cli.command {
            Commands::Config(config_args) => match config_args.action {
                ConfigAction::Get { key } => {
                    assert_eq!(key, "core.root");
                }
                _ => panic!("Expected ConfigAction::Get"),
            },
        }
    }

    #[test]
    fn test_parse_config_get_missing_key() {
        let err = Cli::try_parse_from(["ghwt", "config", "get"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_parse_config_set_with_key_value_and_quiet_flag() {
        let cli = Cli::try_parse_from(["ghwt", "config", "set", "test.key", "test_value", "-q"]).unwrap();
        match cli.command {
            Commands::Config(config_args) => match config_args.action {
                ConfigAction::Set(args) => {
                    assert_eq!(args.key, "test.key");
                    assert_eq!(args.value, "test_value");
                    assert!(args.quiet);
                }
                _ => panic!("Expected ConfigAction::Set"),
            },
        }
    }
}
