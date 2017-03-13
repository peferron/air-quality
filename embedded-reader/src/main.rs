extern crate chrono;
extern crate redis;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serial;

mod args;
mod error;
mod measurement;

use args::Args;
use error::{Error, Result};
use measurement::Measurement;
use redis::Commands;
use serial::*;
use std::time::Duration;
use std::io::{BufRead, BufReader, Lines};
use std::process;

fn main() {
    if let Err(e) = run() {
        match e {
            Error::Args(usage) => println!("{}", usage),
            _ => println!("{:?}", e),
        }
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::from_env()?;
    println!("Starting with {:?}", args);

    let redis_conn = redis::Client::open(&args.redis_url[..])?.get_connection()?;

    for line in read_lines(&args.serial_port)? {
        let measurement = Measurement::from_line(&line?)?;
        let json = measurement.to_json();
        redis_conn.lpush(&args.redis_key, &json)?;
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

    SerialPort::set_timeout(&mut port, Duration::from_secs(90))?;
    
    Ok(BufReader::new(port).lines())
}
