use std::io::Error as IoError;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

use chrono::format::ParseError as ChronoParseError;
use isahc::Error as IsahcError;
use rss::Error as RssError;
use rusqlite::Error as RusqliteError;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Rusqlite(RusqliteError),
    ParseInt(ParseIntError),
    ChronoParse(ChronoParseError),
    Rss(RssError),
    FromUtf8(FromUtf8Error),
    Isahc(IsahcError),
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

impl From<RssError> for Error {
    fn from(error: RssError) -> Self {
        Self::Rss(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::FromUtf8(error)
    }
}

impl From<IsahcError> for Error {
    fn from(error: IsahcError) -> Self {
        Self::Isahc(error)
    }
}
