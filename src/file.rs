use crate::wrkn::Entry;
use chrono::SecondsFormat;
use color_eyre::eyre;
use std::io::Write;

static SEPARATOR: &str = " | ";

pub fn write_wrkn_file(mut out: impl Write, data: &mut [Entry]) -> eyre::Result<()> {
    data.sort_by_key(|entry| entry.timestamp);
    for entry in data {
        let timestamp = entry
            .timestamp
            .and_utc()
            .to_rfc3339_opts(SecondsFormat::Secs, true);
        writeln!(out, "{timestamp}{SEPARATOR}{}", entry.content)?
    }
    Ok(())
}
