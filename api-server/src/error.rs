use hyper;
use serde_json;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    Args(String),
    Hyper(hyper::Error),
    Influx(String),
    Io(io::Error),
    Json(serde_json::Error),
    Measurement(String),
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

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}
