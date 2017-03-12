extern crate serial;

use std::env;
use std::io::*;
use std::process;
use std::time::Duration;

struct Args {
    port: String,
}

#[derive(Debug)]
struct Measurement {
    small_particle_count: i32,
    large_particle_count: i32,
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
    let measurement = parse_line(&line)?;
    enqueue(measurement)
}

fn parse_line(line: &String) -> Result<Measurement> {
    let values: Vec<_> = line.trim().split(',').collect();

    if values.len() == 2 {
        if let (Ok(small), Ok(big)) = (values[0].parse(), values[1].parse()) {
            return Ok(Measurement {
                small_particle_count: small,
                large_particle_count: big,
            })
        }
    }

    Err(Error::new(ErrorKind::InvalidData, format!(
        "Cannot parse measurement for line: \"{}\"", line
    ))) 
}

fn enqueue(measurement: Measurement) -> Result<()> {
    println!("TODO: enqueue measurement {:?}", measurement);
    Ok(())
}
