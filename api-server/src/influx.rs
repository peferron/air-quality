use error::{Error, Result};
use hyper;
use hyper::status::StatusCode;
use measurement::Measurement;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;

pub struct Client {
    http_client: hyper::client::Client,
    write_url: String,
    measurement_name: String,
}

impl Client {
    pub fn new(write_url: &str, measurement_name: &str) -> Client {
        Client {
            http_client: hyper::client::Client::new(),
            write_url: String::from(write_url),
            measurement_name: String::from(measurement_name),
        }
    }

    pub fn write(&self, measurements: &[Measurement]) -> Result<()> {
        let lines: Vec<String> = measurements
            .iter()
            .map(|measurement| serialize_measurement(&self.measurement_name, &measurement))
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

fn serialize_measurement(name: &str, measurement: &Measurement) -> String {
    format!(
        "{name},{tags} {fields} {time}",
        name = name,
        tags = serialize_map(&measurement.tags),
        fields = serialize_map(&measurement.fields),
        time = measurement.time.timestamp() * 1_000_000_000 +
            measurement.time.timestamp_subsec_nanos() as i64
    )
}

fn serialize_map<V>(map: &HashMap<String, V>) -> String where V: Display {
    map
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join(",")
}
