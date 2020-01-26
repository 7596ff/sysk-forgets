use std::io::Error as IoError;
use rusqlite::Error as RusqliteError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Rusqlite(RusqliteError),
    ParseInt(ParseIntError),
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
