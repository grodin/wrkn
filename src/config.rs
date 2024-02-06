use crate::cli::Cli;
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
    pub entries_file: PathBuf,
}

const APPLICATION_NAME: &str = "wrkn";
const DEFAULT_FILE_NAME: &str = "wrkn_file";

impl Config {
    pub(crate) fn from_args(args: &Cli) -> eyre::Result<Self> {
        let entries_file = if let Some(entries_file) = &args.file {
            entries_file.clone()
        } else {
            directories::ProjectDirs::from("", "", APPLICATION_NAME)
                .ok_or_else(|| eyre!("Unable to find user's home directory"))?
                .data_dir()
                .join(Path::new(DEFAULT_FILE_NAME))
        };
        Ok(Self { entries_file })
    }
}
