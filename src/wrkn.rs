use chrono::NaiveDateTime;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entry {
    pub timestamp: NaiveDateTime,
    pub content: String,
}

// impl FromStr for Entry {
//     type Err = Error<String>;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // match parse_entry(s).finish() {
//         //     Ok((_remaining, entry)) => Ok(entry),
//         //     Err(Error { input, code }) => Err(Error {
//         //         input: input.to_string(),
//         //         code,
//         //     }),
//         // }
//         todo!()
//     }
// }
