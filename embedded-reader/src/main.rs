extern crate chrono;
extern crate redis;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serial;

mod args;
mod dylos;
mod error;

use args::Args;
use chrono::Local;
use error::{Error, Result};
use redis::Commands;
use serial::*;
use std::time::Duration;
use std::io::{BufRead, BufReader, Lines};
use std::process;

const REDIS_KEY: &'static str = "measurements";

fn main() {
    if let Err(e) = run() {
        match e {
            Error::Args(usage) => println!("{}", usage),
            _ => println!("Fatal error: {:#?}", e),
        }
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::from_env()?;
    println!("Starting with {:#?}", args);

    let conn = redis::Client::open(&args.redis_url[..])?.get_connection()?;

    for line in read_lines(&args.serial_port)? {
        let json = json!({
            "time": Local::now(),
            "tags": &args.tags,
            "fields": dylos::parse(&line?)?,
        });

        conn.lpush(REDIS_KEY, &json.to_string())?;

        println!("Enqueued measurement {}", json);
    }

    unreachable!();
}

fn read_lines(port_str: &str) -> Result<Lines<BufReader<SystemPort>>> {
    let mut port = open(port_str)?;
    
    port.configure(&PortSettings {
        baud_rate: Baud9600,
        char_size: Bits8,
        parity: ParityNone,
        stop_bits: Stop1,
        flow_control: FlowNone
    })?;

    SerialPort::set_timeout(&mut port, Duration::from_secs(4000))?;
    
    Ok(BufReader::new(port).lines())
}
