use crate::wrkn::{sort_entries_by_timestamp_reverse, Entry};
use color_eyre::eyre;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;

pub fn write_wrkn_file(mut out: impl Write, data: &mut [Entry]) -> eyre::Result<()> {
    data.sort_by_key(sort_entries_by_timestamp_reverse);
    for entry in data {
        writeln!(out, "{entry}")?
    }
    Ok(())
}

pub fn save_wrkn_file<P: AsRef<Path>>(path: P, data: &mut [Entry]) -> eyre::Result<()> {
    let f = BufWriter::new(OpenOptions::new().write(true).create(true).open(&path)?);
    write_wrkn_file(f, data)
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
