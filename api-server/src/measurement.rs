use chrono::{DateTime, Local};

#[derive(Debug, Deserialize)]
pub struct Measurement {
    pub time: DateTime<Local>,
    pub small_particle_count: i32,
    pub large_particle_count: i32,
}
