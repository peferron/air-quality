use error::{Error, Result};
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Args {
    pub serial_port: String,
    pub redis_url: String,
    pub tags: HashMap<String, String>,
}

impl Args {
    pub fn from_env() -> Result<Args> {
        println!("{:#?}", env::args().collect::<Vec<String>>());
        Args::from(env::args())
    }

    pub fn from(mut args: env::Args) -> Result<Args> {
        let path = args.next().unwrap();

        let usage = || Error::Args(format!(
            "Usage: {path} SERIAL_PORT REDIS_URL [TAG_KEY=TAG_VALUE ...]\n\
            Example: {path} /dev/cu.usbserial redis://127.0.0.1 sensor=Dylos_DC_1100_PRO location=home",
            path=path
        ));

        Ok(Args {
            serial_port: args.next().ok_or_else(&usage)?,
            redis_url: args.next().ok_or_else(&usage)?,
            tags: parse_tags(args).ok_or_else(&usage)?,
        })
    }
}

fn parse_tags(args: env::Args) -> Option<HashMap<String, String>> {
    let mut map = HashMap::new();

    for pair in args {
        let mut split = pair.split('=');

        if let (Some(key), Some(value)) = (split.next(), split.next()) {
            map.insert(String::from(key), String::from(value));
        } else {
            return None;
        }
    }

    Some(map)
}
