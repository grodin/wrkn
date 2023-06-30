mod parser;

pub(crate) use parser::SEPARATOR;
use std::cmp::Reverse;
use std::fmt::{Display, Formatter};

use crate::wrkn::parser::parse_wrkn_entry;
use chrono::{DateTime, SecondsFormat, Utc};
use color_eyre::eyre;
use nom::error::Error;
use nom::Finish;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entry {
    pub timestamp: DateTime<Utc>,
    pub title: String,
}

impl FromStr for Entry {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_wrkn_entry(s).finish() {
            Ok((_remaining, entry)) => Ok(entry),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            })?,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let timestamp = self.timestamp.to_rfc3339_opts(SecondsFormat::Secs, true);
        write!(f, "{timestamp}{SEPARATOR}{}", self.title)
    }
}

pub fn sort_entries_by_timestamp_reverse(e: &Entry) -> impl Ord {
    Reverse(e.timestamp)
}

pub fn sort_entries_by_timestamp(e: &Entry) -> impl Ord {
    e.timestamp
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;
    use fake::faker::chrono::en;
    use fake::Fake;
    use proptest::prelude::{any, Just, Strategy};
    use proptest::{prop_assert_eq, proptest};
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    fn entry_strategy() -> impl Strategy<Value = Entry> {
        any::<u64>().prop_flat_map(|n| {
            let mut rng = SmallRng::seed_from_u64(n);
            let timestamp: DateTime<Utc> = en::DateTime().fake_with_rng(&mut rng);
            // We don't care about sub-second precision, so zero out the nanos
            let timestamp = timestamp.with_nanosecond(0u32).unwrap();
            let title = fake::faker::lorem::en::Sentence(5..15).fake_with_rng(&mut rng);
            Just(Entry { timestamp, title }).boxed()
        })
    }

    proptest! {

        #[test]
        fn parse_and_to_string_inverse(e in entry_strategy()){
            let entry: Entry = e.to_string().parse().unwrap();
            prop_assert_eq!(entry, e)
        }
    }
}
