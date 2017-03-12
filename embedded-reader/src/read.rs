use serial::*;
use std::io::{BufRead, BufReader, Result};
use std::time::Duration;

pub fn read_lines<F>(port_str: &str, process_line: F) -> Result<()>
    where F: Fn(&str) -> Result<()> {

    let mut port = open(port_str)?;
    
    port.configure(&PortSettings {
        baud_rate: Baud9600,
        char_size: Bits8,
        parity: ParityNone,
        stop_bits: Stop1,
        flow_control: FlowNone
    })?;

    SerialPort::set_timeout(&mut port, Duration::from_secs(90))?;

    let mut reader = BufReader::new(port);
    let mut buffer = String::new();

    println!("Entering read loop");

    loop {
        reader.read_line(&mut buffer)?;
        process_line(&buffer)?;
        buffer.clear();
    }
}
