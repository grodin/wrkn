use clap::{Parser, Subcommand};

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
    Recent {
        /// How many entries to show
        #[arg(short, long, default_value_t = 10)]
        count: u8,
    },
}
