use crate::doing::{DoingId, Entry};
use chrono::NaiveDateTime;
use nom::bytes::complete::take_till;
use nom::character::complete::{char, digit1, multispace0, multispace1, satisfy};
use nom::combinator::{map, map_res, opt, recognize, value};
use nom::error::ParseError;
use nom::multi::{count, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::{IResult, Parser};

pub(crate) fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let (input, _) = dash(input)?;
    let (input, timestamp) = parse_timestamp(input)?;
    let (input, _) = pipe(input)?;
    let (input, content) = parse_content(input)?;
    let (input, id) = opt(parse_id)(input)?;
    Ok((
        input,
        Entry {
            timestamp,
            content: content.to_string(),
            id,
        },
    ))
}

fn parse_timestamp(input: &str) -> IResult<&str, NaiveDateTime> {
    map_res(
        recognize(separated_pair(parse_date, multispace1, parse_time)),
        |s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M"),
    )(input)
}

fn parse_date(i: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(char('-'), digit1)(i)
}

fn parse_time(i: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(digit1, char(':'), digit1)(i)
}

fn parse_content(input: &str) -> IResult<&str, &str> {
    map(
        terminated(take_till(|c| c == '<'), multispace0),
        |s: &str| s.trim(),
    )(input)
}

fn parse_id(input: &str) -> IResult<&str, DoingId> {
    map(
        ws(delimited(
            char('<'),
            count(satisfy(|c| c.is_ascii_alphanumeric()), 32),
            char('>'),
        )),
        |s| s.iter().collect(),
    )(input)
}

fn dash(input: &str) -> IResult<&str, ()> {
    drop_char_ws('-')(input)
}

fn pipe(input: &str) -> IResult<&str, ()> {
    drop_char_ws('|')(input)
}

/// A combinator that consumes a single char surrounded by whitespace, and returns ()
fn drop_char_ws<'a, E: ParseError<&'a str>>(
    c: char,
) -> impl FnMut(&'a str) -> IResult<&'a str, (), E> {
    ws(value((), char(c)))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use proptest::prelude::*;
    use testresult::TestResult;

    use crate::doing::Entry;
    use crate::parser::*;

    #[test]
    fn can_parse_single_line_with_id() -> TestResult {
        let input = "	- 2023-06-23 17:43 | Trying to get feeds-to-pocket to work on @rpi4 <fd400be709811adec539009fd903f1b9>";
        let (remainder, result) = parse_entry(input)?;
        assert_eq!("", remainder);
        let expected = Entry {
            timestamp: NaiveDate::from_ymd_opt(2023, 6, 23)
                .unwrap()
                .and_hms_opt(17, 43, 0)
                .unwrap(),
            content: "Trying to get feeds-to-pocket to work on @rpi4".to_string(),
            id: Some("fd400be709811adec539009fd903f1b9".to_string()),
        };
        Ok(assert_eq!(result, expected))
    }

    #[test]
    fn can_parse_single_line_without_id() -> TestResult {
        let input = "	- 2023-06-23 17:43 | Trying to get feeds-to-pocket to work on @rpi4";
        let (remainder, result) = parse_entry(input)?;
        assert_eq!("", remainder);
        let expected = Entry {
            timestamp: NaiveDate::from_ymd_opt(2023, 6, 23)
                .unwrap()
                .and_hms_opt(17, 43, 0)
                .unwrap(),
            content: "Trying to get feeds-to-pocket to work on @rpi4".to_string(),
            id: None,
        };
        Ok(assert_eq!(result, expected))
    }

    #[test]
    fn can_parse_timestamp() -> TestResult {
        let input = "2023-06-23 17:43";
        let (remainder, timestamp) = parse_timestamp(input)?;
        assert_eq!("", remainder);
        assert_eq!(
            NaiveDate::from_ymd_opt(2023, 6, 23)
                .unwrap()
                .and_hms_opt(17, 43, 0)
                .unwrap(),
            timestamp
        );
        Ok(())
    }

    proptest! {

        #[test]
        fn can_parse_id(s in "<[a-z0-9]{32}>")  {
            parse_id(&s).unwrap();
        }
    }
}
