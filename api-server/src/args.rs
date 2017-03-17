use error::{Error, Result};
use std::env;

#[derive(Debug)]
pub struct Args {
    pub listen_addr: String,
    pub influx_write_url: String,
    pub influx_line_prefix: String,
}

impl Args {
    pub fn from_env() -> Result<Args> {
        let args: Vec<_> = env::args().collect();

        if args.len() < 4 {
            return Err(Error::Args(format!(
                "Usage: {path} LISTEN_ADDR INFLUX_WRITE_URL INFLUX_LINE_PREFIX\n\
                Example: {path} 127.0.0.1:8080 http://127.0.0.1:8086/write?db=air_quality air_quality,sensor=Dylos_DC_1100_PRO,location=home",
                path=args[0]
            )));
        }

        Ok(Args {
            listen_addr: args[1].clone(),
            influx_write_url: args[2].clone(),
            influx_line_prefix: args[3].clone(),
        })
    }
}
