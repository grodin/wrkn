use crate::cli::RunnableCommand;
use crate::config::Config;
use clap::Args;
use std::fs::canonicalize;
use std::io;
use std::io::{stdout, Write};

#[derive(Debug, Args)]
pub(crate) struct LocateCommand {}

impl RunnableCommand for LocateCommand {
    fn run(self, config: &Config) -> color_eyre::Result<()> {
        let mut stdout = io::BufWriter::new(stdout().lock());
        let canonical_path = canonicalize(&config.entries_file)?;
        writeln!(stdout, "{}", canonical_path.display())?;
        stdout.flush()?;
        Ok(())
    }
}
