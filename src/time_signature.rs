use thiserror::Error;

pub struct TimeSignature {
    // beats per measure
    upper: u8,

    // beat unit
    lower: u8,
}

impl TimeSignature {
    pub fn new(upper: u8, lower: u8) -> Result<Self, InvalidTimeSignatureError> {
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

    pub fn set_beats_per_measure(&mut self, value: u8) -> Result<(), InvalidTimeSignatureError> {
        validate_upper(value).inspect(|_| self.upper = value)
    }
    pub fn set_beat_unit(&mut self, value: u8) -> Result<(), InvalidTimeSignatureError> {
        validate_lower(value).inspect(|_| self.lower = value)
    }
}

fn validate_upper(value: u8) -> Result<(), InvalidTimeSignatureError> {
    if value == 0 {
        Err(InvalidTimeSignatureError::CannotBeZero {
            name: "Upper",
            desc: "beats per measure",
        })
    } else {
        Ok(())
    }
}

fn validate_lower(value: u8) -> Result<(), InvalidTimeSignatureError> {
    if value == 0 {
        Err(InvalidTimeSignatureError::CannotBeZero {
            name: "Lower",
            desc: "beat unit",
        })
    } else if !value.is_power_of_two() {
        Err(InvalidTimeSignatureError::LowerNotPowerOfTwo(value))
    } else {
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum InvalidTimeSignatureError {
    #[error("{name} number ({desc}) cannot be 0.")]
    CannotBeZero {
        name: &'static str,
        desc: &'static str,
    },
    #[error("Lower number (beat unit) must be a power of 2 (provded was {0}).")]
    LowerNotPowerOfTwo(u8),
}
