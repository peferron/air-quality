use error::{Error, Result};
use hyper;
use hyper::status::StatusCode;
use measurement::Measurement;
use std::io::Read;

pub struct Client {
    http_client: hyper::client::Client,
    write_url: String,
}

impl Client {
    pub fn new() -> Client {
        Client {
            http_client: hyper::client::Client::new(),
            write_url: String::from("http://127.0.0.1:8086/write?db=air_quality"),
        }
    }

    pub fn write(&self, measurements: &[Measurement]) -> Result<()> {
        println!("Writing {} measurements", measurements.len());
        
        let lines: Vec<String> = measurements
            .iter()
            .map(|m| format!(
                "air_quality,sensor=Dylos_DC_1100_PRO,location=home {}",
                m.to_influx_string()
            ))
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
