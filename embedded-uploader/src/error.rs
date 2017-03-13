use hyper;
use serde_json;
use std::{io, result};
use redis;

#[derive(Debug)]
pub enum Error {
    Args(String),
    Hyper(hyper::Error),
    Io(io::Error),
    Redis(redis::RedisError),
    Response(Option<String>),
    Serde(serde_json::Error),
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

impl From<redis::RedisError> for Error {
    fn from(e: redis::RedisError) -> Error {
        Error::Redis(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Serde(e)
    }
}
