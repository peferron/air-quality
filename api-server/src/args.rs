use error::{Error, Result};
use std::env;

#[derive(Debug)]
pub struct Args {
    pub listen_addr: String,
    pub influx_write_url: String,
    pub influx_measurement_name: String,
}

impl Args {
    pub fn from_env() -> Result<Args> {
        Args::from(env::args())
    }

    pub fn from(mut args: env::Args) -> Result<Args> {
        let path = args.next().unwrap();

        let usage = || Error::Args(format!(
            "Usage: {path} LISTEN_ADDR INFLUX_WRITE_URL INFLUX_MEASUREMENT_NAME\n\
            Example: {path} 127.0.0.1:8080 http://127.0.0.1:8086/write?db=test air_quality\n\
            Optional: For authentication, set the INFLUX_USERNAME and INFLUX_PASSWORD environment variables.",
            path = path
        ));

        Ok(Args {
            listen_addr: args.next().ok_or_else(&usage)?,
            influx_write_url: args.next().ok_or_else(&usage)?,
            influx_measurement_name: args.next().ok_or_else(&usage)?,
        })
    }
}
