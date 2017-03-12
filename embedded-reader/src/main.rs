extern crate chrono;
extern crate redis;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serial;

mod measurement;
mod read;

use measurement::Measurement;
use redis::Commands;
use std::env;
use std::io::{Error, ErrorKind, Result};
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, format!(
            "Usage: {path} PORT\n\
             Example: {path} /dev/cu.usbserial",
             path=args[0]
        )));
    }

    let port = &args[1];
    read::read_lines(port, process_line)
}

fn process_line(line: &str) -> Result<()> {
    match Measurement::from_line(&line) {
        Some(measurement) => enqueue(measurement).map_err(|e| Error::new(ErrorKind::Other, e)),
        None => Err(Error::new(ErrorKind::InvalidData, format!(
            "Cannot parse measurement for line: \"{}\"", line
        ))),
    }
}

fn enqueue(measurement: Measurement) -> redis::RedisResult<()> {
    let json = measurement.to_json();

    let result = redis::Client::open("redis://127.0.0.1/")?
        .get_connection()?
        .rpush("measurements", &json);
    
    if result.is_ok() {
        println!("Enqueued measurement {}", json);
    }

    result
}
