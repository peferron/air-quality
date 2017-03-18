use chrono::{DateTime, Local};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Measurement<'a> {
    pub tags: &'a HashMap<String, String>,
    pub fields: &'a HashMap<String, f64>,
    pub time: DateTime<Local>,
}
