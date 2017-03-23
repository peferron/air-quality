use error::{Error, Result};
use hyper;
use hyper::header::{Authorization, Basic};
use hyper::status::StatusCode;
use measurement::Measurement;
use std::collections::HashMap;
use std::env;
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

        let mut req = self.http_client
            .post(&self.write_url)
            .body(&req_body);

        if let (Ok(u), Ok(p)) = (env::var("INFLUX_USERNAME"), env::var("INFLUX_PASSWORD")) {
            println!("Authenticating as user \"{}\"",u);
            req = req.header(Authorization(Basic { username: u, password: Some(p) }));
        }

        let mut res = req.send()?;
        let mut res_body = String::new();
        res.read_to_string(&mut res_body)?;

        println!("Received response from Influx: {} {}", res.status, res_body);

        match res.status {
            StatusCode::NoContent => Ok(()),
            _ => Err(Error::Influx(format!("{} {}", res.status, res_body)))
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
