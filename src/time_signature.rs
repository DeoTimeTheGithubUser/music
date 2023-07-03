use thiserror::Error;

pub struct TimeSignature {
    // beats per measure
    upper: u8,

    // beat unit
    lower: u8,
}

impl TimeSignature {
    pub fn new(upper: u8, lower: u8) -> Result<Self, Error> {
        validate_upper(upper)?;
        validate_lower(lower)?;
        Ok(TimeSignature { upper, lower })
    }

    pub fn beats_per_measure(&self) -> u8 {
        self.upper
    }

    pub fn beat_unit(&self) -> u8 {
        self.lower
    }

    pub fn set_beats_per_measure(&mut self, value: u8) -> Result<(), Error> {
        validate_upper(value).map(|_| self.upper = value)
    }
    pub fn set_beat_unit(&mut self, value: u8) -> Result<(), Error> {
        validate_lower(value).map(|_| self.lower = value)
    }
}

fn validate_upper(value: u8) -> Result<(), Error> {
    match value {
        0 => Err(Error::CannotBeZero {
            name: "Upper",
            desc: "beats per measure",
        }),
        _ => Ok(()),
    }
}

fn validate_lower(value: u8) -> Result<(), Error> {
    match value {
        0 => Err(Error::CannotBeZero {
            name: "Lower",
            desc: "beat unit",
        }),
        _ if !value.is_power_of_two() => Err(Error::LowerNotPowerOfTwo(value)),
        _ => Ok(()),
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("{name} number ({desc}) cannot be 0.")]
    CannotBeZero {
        name: &'static str,
        desc: &'static str,
    },
    #[error("Lower number (beat unit) must be a power of 2 (provded was {0}).")]
    LowerNotPowerOfTwo(u8),
}
