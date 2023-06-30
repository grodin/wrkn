use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
    pub wrkn_file: PathBuf,
}

static DEFAULT_FILE_NAME: &str = "wrkn_file";

impl Default for Config {
    fn default() -> Self {
        let wrkn_file = directories::ProjectDirs::from("", "", "wrkn")
            .expect("Couldn't find Home directory")
            .data_dir()
            .to_path_buf()
            .join(Path::new(DEFAULT_FILE_NAME));
        Self { wrkn_file }
    }
}
