use error::{Error, Result};
use hyper;
use hyper::status::StatusCode;
use measurement::Measurement;
use std::io::Read;

pub struct Client {
    http_client: hyper::client::Client,
    write_url: String,
    influx_measurement_name: String,
}

impl Client {
    pub fn new(write_url: &str, influx_measurement_name: &str) -> Client {
        Client {
            http_client: hyper::client::Client::new(),
            write_url: String::from(write_url),
            influx_measurement_name: String::from(influx_measurement_name),
        }
    }

    pub fn write(&self, measurements: &[Measurement]) -> Result<()> {      
        let lines: Vec<String> = measurements
            .iter()
            .map(|m| format!("{},{}", self.influx_measurement_name, m.to_influx_string()))
            .collect();

        println!("Writing {} measurements to Influx, last: {}",
            lines.len(), lines.last().unwrap_or(&String::from("None")));

        let req_body = lines.join("\n");

        let mut response = self.http_client.post(&self.write_url).body(&req_body).send()?;
        let mut response_body = String::new();
        response.read_to_string(&mut response_body)?;

        println!("Received response from Influx: {} {}", response.status, response_body);

        match response.status {
            StatusCode::NoContent => Ok(()),
            _ => Err(Error::Influx(format!("{} {}", response.status, response_body)))
        }
    }
}
