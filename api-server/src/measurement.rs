use chrono::{DateTime, Local};

#[derive(Debug, Deserialize)]
pub struct Measurement {
    pub time: DateTime<Local>,
    pub small_particle_count: i32,
    pub large_particle_count: i32,
}

impl Measurement {
    pub fn to_influx_string(&self) -> String {
        format!(
            "small_particle_count={s},large_particle_count={l} {t}",
            s=self.small_particle_count,
            l=self.large_particle_count,
            t=self.time.timestamp() * 1_000_000_000 + self.time.timestamp_subsec_nanos() as i64,
        )
    }
}