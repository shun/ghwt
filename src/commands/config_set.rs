use crate::config::Config;
use anyhow::Result;

#[derive(clap::Args, Debug, Clone)]
pub struct SetArgs {
    /// The key to set
    #[arg(value_name = "KEY")]
    pub key: String,
    /// The value to set
    #[arg(value_name = "VALUE")]
    pub value: String,
    /// Do not print anything to stdout
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn handle_config_set(args: SetArgs) -> Result<()> {
    let mut config = Config::load().map_err(|e| anyhow::anyhow!(e))?;
    config.set_value(&args.key, &args.value)?;

    let config_path = Config::get_config_path()?;

    config.save(&config_path)?;

    if !args.quiet {
        println!("Key '{}' set to value '{}'", args.key, args.value);
    }

    Ok(())
}
