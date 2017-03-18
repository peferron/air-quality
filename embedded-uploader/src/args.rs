use error::{Error, Result};
use std::env;

#[derive(Debug)]
pub struct Args {
    pub redis_url: String,
    pub api_url: String,
}

impl Args {
    pub fn from_env() -> Result<Args> {
        let args: Vec<_> = env::args().collect();

        if args.len() < 5 {
            return Err(Error::Args(format!(
                "Usage: {path} REDIS_URL API_URL\n\
                Example: {path} redis://127.0.0.1 http://example.com/air-quality/api",
                path=args[0]
            )));
        }

        Ok(Args {
            redis_url: args[1].clone(),
            api_url: args[4].clone(),
        })
    }
}
