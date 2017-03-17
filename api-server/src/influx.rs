use error::{Error, Result};
use hyper;
use hyper::status::StatusCode;
use measurement::Measurement;
use std::io::Read;

pub struct Client {
    http_client: hyper::client::Client,
    write_url: String,
    line_prefix: String,
}

impl Client {
    pub fn new(write_url: &str, line_prefix: &str) -> Client {
        Client {
            http_client: hyper::client::Client::new(),
            write_url: String::from(write_url),
            line_prefix: String::from(line_prefix),
        }
    }

    pub fn write(&self, measurements: &[Measurement]) -> Result<()> {
        println!("Writing {} measurements", measurements.len());
        
        let lines: Vec<String> = measurements
            .iter()
            .map(|m| format!("{} {}", self.line_prefix, m.to_influx_string()))
            .collect();

        let req_body = lines.join("\n");

        let mut response = self.http_client.post(&self.write_url).body(&req_body).send()?;
        let mut response_body = String::new();
        response.read_to_string(&mut response_body)?;

        println!("Received response: {} {}", response.status, response_body);

        match response.status {
            StatusCode::NoContent => Ok(()),
            _ => Err(Error::Influx(format!("{} {}", response.status, response_body)))
        }
    }
}
