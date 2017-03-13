use chrono::{DateTime, Local};
use error::Error;
use serde_json;
use std::io;

#[derive(Serialize)]
pub struct Measurement {
    pub time: DateTime<Local>,
    pub small_particle_count: i32,
    pub large_particle_count: i32,
}

impl Measurement {
    pub fn from_line(line: &str) -> Result<Measurement, Error> {
        let values: Vec<_> = line.trim().split(',').collect();

        if values.len() == 2 {
            if let (Ok(small), Ok(big)) = (values[0].parse(), values[1].parse()) {
                return Ok(Measurement {
                    time: Local::now(),
                    small_particle_count: small,
                    large_particle_count: big,
                });
            }
        }

        Err(Error::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Cannot parse measurement for line: \"{}\"", line)
        )))
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
