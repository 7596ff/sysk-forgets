use std::io::Error as IoError;
use std::num::ParseIntError;

use chrono::format::ParseError as ChronoParseError;
use rusqlite::Error as RusqliteError;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Rusqlite(RusqliteError),
    ParseInt(ParseIntError),
    ChronoParse(ChronoParseError),
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

impl From<RusqliteError> for Error {
    fn from(error: RusqliteError) -> Self {
        Self::Rusqlite(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

impl From<ChronoParseError> for Error {
    fn from(error: ChronoParseError) -> Self {
        Self::ChronoParse(error)
    }
}
