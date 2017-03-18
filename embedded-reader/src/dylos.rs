use error::{Error, Result};
use std::collections::HashMap;

pub fn parse(line: &str) -> Result<HashMap<String, f64>> {
    let values: Vec<_> = line.trim().split(',').collect();

    if values.len() == 2 {
        if let (Ok(small), Ok(big)) = (values[0].parse(), values[1].parse()) {
            let mut fields = HashMap::new();
            fields.insert(String::from("small_particle_count"), small);
            fields.insert(String::from("large_particle_count"), big);
            return Ok(fields);
        }
    }

    Err(Error::Dylos(format!("Cannot parse Dylos fields for line: \"{}\"", line)))
}
