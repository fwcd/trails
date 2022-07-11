use std::{string::FromUtf8Error, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Reqwest(String),
    UrlParse(String),
    UnsupportedScheme(String),
    UnexpectedToken(String),
    Utf8(String),
    Io(String),
    Eof,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self { Self::Reqwest(format!("{:?}", e)) }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self { Self::UrlParse(format!("{:?}", e)) }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self { Self::Utf8(format!("{:?}", e)) }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self { Self::Io(format!("{:?}", e)) }
}
