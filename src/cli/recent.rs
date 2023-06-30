use crate::cli::RunnableCommand;
use crate::config::Config;
use clap::Args;
use std::fs::File;
use std::io;
use wrkn::file;

#[derive(Debug, Args)]
pub(crate) struct RecentCommand {
    /// How many entries to show
    #[arg(short, long, default_value_t = 10)]
    pub(crate) count: u8,
}

impl RunnableCommand for RecentCommand {
    fn run(self, config: &Config) -> color_eyre::Result<()> {
        dbg!(&self, &config);
        if config.wrkn_file.try_exists()? {
            let mut f = io::BufReader::new(File::open(&config.wrkn_file)?);
            file::read_wrkn_file(&mut f)?
                .iter()
                .take(self.count as usize)
                .for_each(|entry| println!("{}", entry))
        }
        Ok(())
    }
}
