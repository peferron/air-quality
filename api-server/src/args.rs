use error::{Error, Result};
use std::env;

#[derive(Debug)]
pub struct Args {
    pub listen_addr: String,
}

impl Args {
    pub fn from_env() -> Result<Args> {
        let args: Vec<_> = env::args().collect();

        if args.len() < 2 {
            return Err(Error::Args(format!(
                "Usage: {path} LISTEN_ADDR\n\
                Example: {path} 0.0.0.0:8080",
                path=args[0]
            )));
        }

        Ok(Args {
            listen_addr: args[1].clone(),
        })
    }
}
