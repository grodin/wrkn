use crate::cli::RunnableCommand;
use crate::config::Config;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct OpenCommand {}

impl RunnableCommand for OpenCommand {
    fn run(self, config: &Config) -> color_eyre::Result<()> {
        edit::edit_file(&config.entries_file)?;
        Ok(())
    }
}
