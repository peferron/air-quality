use error::{Error, Result};
use std::env;

#[derive(Debug)]
pub struct Args {
    pub redis_url: String,
    pub api_url: String,
}

impl Args {
    pub fn from_env() -> Result<Args> {
        Args::from(env::args())
    }

    pub fn from(mut args: env::Args) -> Result<Args> {
        let path = args.next().unwrap();

        let usage = || Error::Args(format!(
                "Usage: {path} REDIS_URL API_URL\n\
                Example: {path} redis://127.0.0.1 http://example.com/air-quality/api/measurements",
                path = path
        ));

        Ok(Args {
            redis_url: args.next().ok_or_else(&usage)?,
            api_url: args.next().ok_or_else(&usage)?,
        })
    }
}
