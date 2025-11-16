use crate::wrkn::{sort_entries_by_timestamp_reverse, Entry};
use atomicwrites::{AtomicFile, OverwriteBehavior};
use color_eyre::eyre;
use color_eyre::eyre::Context;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::{fs, io};

pub fn save_wrkn_file<P: AsRef<Path>>(path: P, data: &mut [Entry]) -> eyre::Result<()> {
    let path = path.as_ref().to_path_buf();

    // If path doesn't exist then we can't canonicalize it
    // But that doesn't matter for the symlink resolving below since if it doesn't exist
    // then it can't be a symlink
    let path = if path.exists() {
        path.canonicalize()?
    } else {
        path
    };

    // If path is a symlink, resolve the symlink so we atomically write to the target
    let resolved_path;
    let path = if path.is_symlink() {
        resolved_path = fs::read_link(&path)
            .with_context(|| format!("Failed to read link: {}", &path.display()))?
            .canonicalize()?;
        resolved_path
    } else {
        path
    };

    let af = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);
    af.write(|f| {
        data.sort_by_key(sort_entries_by_timestamp_reverse);
        for entry in data {
            writeln!(f, "{entry}")?
        }
        Ok::<(), io::Error>(())
    })?;
    Ok(())
}

pub fn read_wrkn_file<P: AsRef<Path>>(path: P) -> eyre::Result<Vec<Entry>> {
    if path.as_ref().try_exists()? {
        let mut entries: Vec<Entry> = io::BufReader::new(File::open(&path)?)
            .lines()
            .map(|l| l?.parse())
            .collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(sort_entries_by_timestamp_reverse);
        Ok(entries)
    } else {
        Ok(vec![])
    }
}
