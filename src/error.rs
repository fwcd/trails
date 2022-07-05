use std::iter::from_fn;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Reqwest(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self { Self::Reqwest(format!("{:?}", e)) }
}
