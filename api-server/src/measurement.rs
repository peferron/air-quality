use chrono::{DateTime, Local};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Measurement {
    pub tags: HashMap<String, String>,
    pub fields: HashMap<String, f64>,
    pub time: DateTime<Local>,
}
