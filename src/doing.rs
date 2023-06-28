use crate::parser::parse_entry;
use chrono::NaiveDateTime;
use nom::error::Error;
use nom::Finish;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entry {
    pub timestamp: NaiveDateTime,
    pub content: String,
    pub id: Option<DoingId>,
}

pub type DoingId = String;

impl FromStr for Entry {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_entry(s).finish() {
            Ok((_remaining, entry)) => Ok(entry),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}
