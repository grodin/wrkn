use crate::cli::RunnableCommand;
use crate::config::Config;
use clap::Args;
use itertools::Itertools;
use wrkn::file;
use wrkn::wrkn::{sort_entries_by_timestamp, Entry};

#[derive(Debug, Args)]
pub(crate) struct RecentCommand {
    /// How many entries to show
    #[arg(short, long, default_value_t = 10)]
    pub(crate) count: u8,
}

impl RunnableCommand for RecentCommand {
    fn run(self, config: &Config) -> color_eyre::Result<()> {
        let entries: Vec<Entry> = file::read_wrkn_file(&config.wrkn_file)?
            .into_iter()
            .take(self.count as usize)
            .sorted_by_key(sort_entries_by_timestamp)
            .collect();
        Ok(entries.iter().for_each(|entry| println!("{}", entry)))
    }
}
