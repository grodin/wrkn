use crate::cli::Cli;
use crate::config::Config;
use clap::Parser;
use color_eyre::eyre;

mod cli;
mod config;
mod file;
mod wrkn;

fn main() -> eyre::Result<()> {
    let args = Cli::parse();
    #[cfg(debug_assertions)]
    dbg!(&args);
    let config = Config::from_args(&args)?;
    args.command.unwrap_or_default().run(&config)
}
