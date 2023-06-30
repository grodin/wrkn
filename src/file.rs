use crate::wrkn::Entry;
use color_eyre::eyre;
use std::cmp::Reverse;
use std::io::{BufRead, Write};

pub fn write_wrkn_file(mut out: impl Write, data: &mut [Entry]) -> eyre::Result<()> {
    data.sort_by_key(sort_entries_by_timestamp_reverse);
    for entry in data {
        writeln!(out, "{entry}")?
    }
    Ok(())
}

pub fn read_wrkn_file(input: &mut (impl BufRead + ?Sized)) -> eyre::Result<Vec<Entry>> {
    let mut entries: Vec<Entry> = input
        .lines()
        .map(|l| l?.parse())
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(sort_entries_by_timestamp_reverse);
    Ok(entries)
}

fn sort_entries_by_timestamp_reverse(e: &Entry) -> impl Ord {
    Reverse(e.timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use once_cell::sync::Lazy;
    use pretty_assertions::assert_eq;
    use testresult::TestResult;

    static WRKN_FILE: &str = "1979-06-12T01:34:00Z | Something or other";

    static ENTRIES: Lazy<Vec<Entry>> = Lazy::new(|| {
        vec![Entry {
            timestamp: Utc.with_ymd_and_hms(1979, 6, 12, 1, 34, 00).unwrap(),
            title: "Something or other".to_string(),
        }]
    });

    #[test]
    fn can_read_multi_line_file() -> TestResult {
        let mut buf = WRKN_FILE.as_bytes();
        let entries = read_wrkn_file(&mut buf)?;
        assert_eq!(ENTRIES.as_ref(), entries);
        Ok(())
    }
}
