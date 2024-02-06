use crate::cli::RunnableCommand;
use crate::config::Config;
use crate::file::read_wrkn_file;
use crate::wrkn::{sort_entries_by_timestamp, Entry};
use chrono_humanize::HumanTime;
use clap::Args;
use itertools::Itertools;
use owo_colors::{OwoColorize, Stream};
use std::io;
use std::io::{stdout, Write};

#[derive(Debug, Args)]
pub(crate) struct RecentCommand {
    /// How many entries to show
    #[arg(short, long, default_value_t = 10)]
    pub(crate) count: u8,
}

impl RunnableCommand for RecentCommand {
    fn run(self, config: &Config) -> color_eyre::Result<()> {
        let entries: Vec<Entry> = read_wrkn_file(&config.entries_file)?
            .into_iter()
            .take(self.count as usize)
            .sorted_by_key(sort_entries_by_timestamp)
            .collect();
        let mut stdout = io::BufWriter::new(stdout().lock());
        for entry in entries {
            pretty_print(&mut stdout, &entry)?;
        }
        stdout.flush()?;
        Ok(())
    }
}

fn pretty_print(writer: &mut impl Write, entry: &Entry) -> color_eyre::Result<()> {
    let pretty_timestamp = format!(
        "{:<14}",
        HumanTime::from(entry.timestamp)
            .if_supports_color(Stream::Stdout, |text| text.bright_black())
    );
    let pretty_separator = " | ".if_supports_color(Stream::Stdout, |text| text.cyan());
    let pretty_title = &entry.title;
    writeln!(writer, "{pretty_timestamp}{pretty_separator}{pretty_title}")?;
    Ok(())
}
