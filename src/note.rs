use std::fmt::{Display, Formatter};

use crate::{Accidental, Pitch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pitch: Pitch,
    accidental: Accidental,
    octave: u8,
}

impl Note {
    pub fn pitch(&self) -> Pitch {
        self.pitch
    }
    pub fn accidental(&self) -> Accidental {
        self.accidental
    }
    pub fn octave(&self) -> u8 {
        self.octave
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.pitch, f)?;
        Display::fmt(&self.accidental, f)?;
        Display::fmt(&self.octave, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note() {
        let note = Note {
            pitch: Pitch::A,
            accidental: Accidental::Flat,
            octave: 4,
        };
        assert_eq!(format!("{note}"), "A♭4");
    }
}
