pub const REDIS_KEY: &'static str = "measurements";

#[derive(Debug)]
pub struct Measurement {
    pub small_particle_count: i32,
    pub large_particle_count: i32,
}

impl Measurement {
    pub fn from_string(s: &String) -> Option<Measurement> {
        let values: Vec<_> = s.trim().split(',').collect();

        if values.len() != 2 {
            return None;
        }

        match (values[0].parse(), values[1].parse()) {
            (Ok(small), Ok(big)) => Some(Measurement {
                small_particle_count: small,
                large_particle_count: big,
            }),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{}", self.small_particle_count, self.large_particle_count)
    }
}
