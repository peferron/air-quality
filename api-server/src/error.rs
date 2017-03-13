use hyper;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    Args(String),
    Hyper(hyper::Error),
    Io(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Error {
        Error::Hyper(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
