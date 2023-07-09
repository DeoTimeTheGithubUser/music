use crate::{Duration, Pitch};
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Note {
    octave: u8,
    pitch: Pitch,
    duration: Duration,
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.pitch, self.octave)
    }
}
