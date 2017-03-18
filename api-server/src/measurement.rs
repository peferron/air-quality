use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct Measurement {
    pub tags: HashMap<String, String>,
    pub fields: HashMap<String, f64>,
    pub time: DateTime<Local>,
}

impl Measurement {
    pub fn to_influx_string(&self) -> String {
        format!(
            "{tags} {fields} {time}",
            tags = to_influx_string(&self.tags),
            fields = to_influx_string(&self.fields),
            time = self.time.timestamp() * 1_000_000_000 + self.time.timestamp_subsec_nanos() as i64
        )
    }
}

fn to_influx_string<V>(map: &HashMap<String, V>) -> String where V: Display {
    map
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join(",")
}
