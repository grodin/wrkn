use crate::cli::Cli;
use crate::config::Config;
use clap::Parser;
use color_eyre::eyre;

mod cli;
mod config;

fn main() -> eyre::Result<()> {
    let args = Cli::parse();
    #[cfg(debug_assertions)]
    dbg!(&args);
    let config = Config::default();
    args.command.unwrap_or_default().run(&config)
}
