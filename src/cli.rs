use crate::cli::now::NowCommand;
use crate::cli::open::OpenCommand;
use crate::cli::recent::RecentCommand;
use crate::config::Config;
use clap::{Parser, Subcommand};
use color_eyre::eyre;

mod now;
mod open;
mod recent;

/// Simple tool to record what I was just doing
#[derive(Debug, Parser)]
#[command(author, version, about, infer_subcommands = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Shows most recent entries
    Recent(RecentCommand),
    /// Adds an entry with the current time
    Now(NowCommand),
    /// Opens the wrkn file in the default editor
    Open(OpenCommand),
}

impl Command {
    pub fn run(self, config: &Config) -> eyre::Result<()> {
        #[cfg(debug_assertions)]
        dbg!(&self, &config);
        match self {
            Self::Recent(recent) => recent.run(config),
            Self::Now(now) => now.run(config),
            Self::Open(open) => open.run(config),
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Command::Recent(RecentCommand { count: 10 })
    }
}

trait RunnableCommand {
    fn run(self, config: &Config) -> eyre::Result<()>;
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
