use chrono::{DateTime, Local};
use error::{Error, Result};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Measurement {
    pub tags: HashMap<String, String>,
    pub fields: HashMap<String, f64>,
    pub time: DateTime<Local>,
}

impl Measurement {
    pub fn fill(&mut self) -> Result<()> {
        let sensor = self.tags
            .get("sensor")
            .ok_or(Error::Measurement(String::from("Missing 'sensor' tag")))?
            .clone();

        match sensor.as_ref() {
            "Dylos_DC1100_PRO" => self.fill_dylos_dc1100_pro(),
            _ => Err(Error::Measurement(format!("Unsupported sensor: {}", sensor))),
        }
    }

    fn fill_dylos_dc1100_pro(&mut self) -> Result<()> {
        let small = self.fields
            .get("small_particle_count")
            .ok_or(Error::Measurement(String::from(
                "Missing 'small_particle_count' field for Dylos DC1100 PRO"
            )))?
            .clone();

        let aqi = 3.31E-22 * small.powi(5)
            - 1.04E-16 * small.powi(4)
            + 1.19E-11 * small.powi(3)
            - 5.85E-07 * small.powi(2)
            + 1.56E-02 * small
            + 9.43E+00;

        self.fields.insert(String::from("computed_aqi"), aqi.round());

        Ok(())
    }
}
