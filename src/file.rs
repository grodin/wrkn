use crate::wrkn::{sort_entries_by_timestamp_reverse, Entry};
use atomicwrites::{AtomicFile, OverwriteBehavior};
use color_eyre::eyre;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

pub fn save_wrkn_file<P: AsRef<Path>>(path: P, data: &mut [Entry]) -> eyre::Result<()> {
    let af = AtomicFile::new(&path, OverwriteBehavior::AllowOverwrite);
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
