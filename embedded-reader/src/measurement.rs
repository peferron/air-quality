use chrono::{DateTime, Local};
use serde_json;

#[derive(Serialize)]
pub struct Measurement {
    pub time: DateTime<Local>,
    pub small_particle_count: i32,
    pub large_particle_count: i32,
}

impl Measurement {
    pub fn from_line(line: &str) -> Option<Measurement> {
        let values: Vec<_> = line.trim().split(',').collect();

        if values.len() != 2 {
            return None;
        }

        match (values[0].parse(), values[1].parse()) {
            (Ok(small), Ok(big)) => Some(Measurement {
                time: Local::now(),
                small_particle_count: small,
                large_particle_count: big,
            }),
            _ => None,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
