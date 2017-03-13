use std::{io, result};
use serial;
use redis;

#[derive(Debug)]
pub enum Error {
    Args(String),
    Io(io::Error),
    Redis(redis::RedisError),
    Serial(serial::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<serial::Error> for Error {
    fn from(e: serial::Error) -> Error {
        Error::Serial(e)
    }
}

impl From<redis::RedisError> for Error {
    fn from(e: redis::RedisError) -> Error {
        Error::Redis(e)
    }
}
