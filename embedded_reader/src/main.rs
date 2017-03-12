extern crate redis;
use redis::Commands;

extern crate serial;

mod measurement;
use measurement::Measurement;

use std::env;
use std::io::*;
use std::process;
use std::time::Duration;

struct Args {
    port: String,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = parse_args()?;
    let mut port = serial::open(&args.port)?;
    configure(&mut port)?;
    read(&mut port)
}

fn parse_args() -> Result<Args> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, format!(
            "Usage: {path} PORT\n\
             Example: {path} /dev/cu.usbserial",
             path=args[0]
        )));
    }

    Ok(Args {
        port: args[1].clone(),
    })
}

fn configure<T: serial::SerialPort>(port: &mut T) -> serial::Result<()> {
    port.configure(&serial::PortSettings {
        baud_rate: serial::Baud9600,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone
    })?;

    port.set_timeout(Duration::from_secs(90))
}

fn read<T: serial::SerialPort>(port: &mut T) -> Result<()> {
    let mut reader = BufReader::new(port);
    let mut buffer = String::new();

    loop {
        reader.read_line(&mut buffer)?;
        process_line(&buffer)?;
        buffer.clear();
    }
}

fn process_line(line: &String) -> Result<()> {
    match Measurement::from_string(&line) {
        Some(measurement) => enqueue(measurement).map_err(|e| Error::new(ErrorKind::Other, e)),
        None => Err(Error::new(ErrorKind::InvalidData, format!(
            "Cannot parse measurement for line: \"{}\"", line
        ))),
    }
}

fn enqueue(measurement: Measurement) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let connection = client.get_connection()?;
    let result = connection.rpush(measurement::REDIS_KEY, measurement.to_string());
    
    if result.is_ok() {
        println!("Enqueued measurement {}", measurement.to_string());
    }

    result
}
