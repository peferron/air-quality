use error::Error;

#[derive(Serialize)]
pub struct Fields {
    small_particle_count: i32,
    large_particle_count: i32,
}

pub fn parse(line: &str) -> Result<Fields, Error> {
    let values: Vec<_> = line.trim().split(',').collect();

    if values.len() == 2 {
        if let (Ok(small), Ok(big)) = (values[0].parse(), values[1].parse()) {
            return Ok(Fields {
                small_particle_count: small,
                large_particle_count: big,
            });
        }
    }

    Err(Error::Dylos(format!("Cannot parse Dylos fields for line: \"{}\"", line)))
}
