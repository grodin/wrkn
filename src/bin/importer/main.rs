use crate::doing_parser::parse_entry;
use color_eyre::eyre;
use nom::error::Error;
use nom::Finish;
use std::io;
use std::io::BufRead;
use wrkn::file::write_wrkn_file;
use wrkn::wrkn::Entry;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let mut entries: Vec<Entry> = io::stdin()
        .lock()
        .lines()
        //TODO: Make this not a hardcoded skip
        .skip(2) // Skip the lines which don't contain useful data
        .map(|line| parse_doing_entry(&line?))
        .collect::<eyre::Result<Vec<_>>>()?;
    write_wrkn_file(io::stdout().lock(), &mut entries)?;
    Ok(())
}

fn parse_doing_entry(s: &str) -> eyre::Result<Entry> {
    match parse_entry(s).finish() {
        Ok((_remaining, entry)) => Ok(entry),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        })?,
    }
}

mod doing_parser;
