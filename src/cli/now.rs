use crate::cli::RunnableCommand;
use crate::config::Config;
use chrono::{Timelike, Utc};
use clap::Args;
use itertools::Itertools;
use wrkn::file::{read_wrkn_file, save_wrkn_file};
use wrkn::wrkn::Entry;

#[derive(Debug, Args)]
pub(crate) struct NowCommand {
    /// The entry to be added
    #[arg(trailing_var_arg = true)]
    pub title: Vec<String>,
}

impl RunnableCommand for NowCommand {
    fn run(self, config: &Config) -> color_eyre::Result<()> {
        let mut entries = read_wrkn_file(&config.wrkn_file)?;
        let new_entry = Entry {
            timestamp: Utc::now().with_nanosecond(0).unwrap(),
            title: self.title.into_iter().join(" "),
        };
        dbg!(&new_entry);
        entries.push(new_entry.clone());
        save_wrkn_file(&config.wrkn_file, &mut entries)?;
        println!(
            "Added \"{}\" to wrkn file {} at {}",
            &new_entry.title,
            &config.wrkn_file.display(),
            &new_entry.timestamp
        );
        Ok(())
    }
}
