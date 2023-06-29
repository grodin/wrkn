use crate::cli::{Cli, Command};
use clap::Parser;
use color_eyre::eyre;

mod cli;

fn main() -> eyre::Result<()> {
    let args = Cli::parse();
    println!("{:?}", args);
    match args.command.unwrap_or(Command::Recent { count: 10 }) {
        Command::Recent { count } => run_recent(count),
    }
}

fn run_recent(count: u8) -> eyre::Result<()> {
    println!("printing {} most recent entries", count);
    Ok(())
}
