use crate::wrkn::Entry;
use chrono::{DateTime, Utc};
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::char;
use nom::combinator::{map_res, recognize, rest, value};
use nom::sequence::terminated;
use nom::IResult;

pub(crate) static SEPARATOR: &str = " | ";

pub(crate) fn parse_wrkn_entry(input: &str) -> IResult<&str, Entry> {
    let (input, timestamp) = datetime(input)?;
    let (input, _) = pipe(input)?;
    let (remainder, title) = title(input)?;
    Ok((
        remainder,
        Entry {
            timestamp,
            title: title.to_string(),
        },
    ))
}

fn datetime(i: &str) -> IResult<&str, DateTime<Utc>> {
    map_res(
        recognize(terminated(is_a("01234567890-:T"), char('Z'))),
        |s: &str| s.parse(),
    )(i)
}

fn pipe(i: &str) -> IResult<&str, ()> {
    value((), tag(" | "))(i)
}

fn title(i: &str) -> IResult<&str, &str> {
    rest(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use pretty_assertions::assert_eq;
    use testresult::TestResult;

    static ENTRY_LINE: &str = "2020-12-18T11:12:00Z | Investigating Orbit-MVI library for architecture @map-librarian @android";

    #[test]
    fn can_parse_single_line_to_entry() -> TestResult {
        let (_remaining, entry) = parse_wrkn_entry(ENTRY_LINE)?;
        let expected = Entry {
            timestamp: Utc.with_ymd_and_hms(2020, 12, 18, 11, 12, 0).unwrap(),
            title: "Investigating Orbit-MVI library for architecture @map-librarian @android"
                .to_string(),
        };
        assert_eq!(expected, entry);
        Ok(())
    }
}
