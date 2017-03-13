use error::{Error, Result};
use std::env;

#[derive(Debug)]
pub struct Args {
    pub serial_port: String,
    pub redis_url: String,
    pub redis_key: String,
}

impl Args {
    pub fn from_env() -> Result<Args> {
        let args: Vec<_> = env::args().collect();

        if args.len() < 4 {
            return Err(Error::Args(format!(
                "Usage: {path} SERIAL_PORT REDIS_URL REDIS_KEY\n\
                Example: {path} /dev/cu.usbserial redis://127.0.0.1 measurements",
                path=args[0]
            )));
        }

        Ok(Args {
            serial_port: args[1].clone(),
            redis_url: args[2].clone(),
            redis_key: args[3].clone(),
        })
    }
}
