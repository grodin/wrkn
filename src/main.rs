use crate::cli::Cli;
use crate::config::Config;
use clap::Parser;
use color_eyre::eyre;

mod cli;
mod config;

fn main() -> eyre::Result<()> {
    let args = Cli::parse();
    println!("{:?}", args);
    let config = Config::default();
    args.command.unwrap_or_default().run(&config)
}
