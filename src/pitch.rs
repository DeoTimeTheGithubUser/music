use std::fmt::{Debug, Display, Formatter, Write};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pitch {
    pitch: Name,
    accidental: Accidental,
    octave: u8,
}

// Todo: Setters maybe
impl Pitch {
    pub fn name(&self) -> Name {
        self.pitch
    }
    pub fn accidental(&self) -> Accidental {
        self.accidental
    }
    pub fn octave(&self) -> u8 {
        self.octave
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, strum::Display)]
pub enum Name {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Name {
    type Error = NameError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'a' => Ok(Name::A),
            'b' => Ok(Name::B),
            'c' => Ok(Name::C),
            'd' => Ok(Name::D),
            'e' => Ok(Name::E),
            'f' => Ok(Name::F),
            'g' => Ok(Name::G),
            _ => Err(NameError(value)),
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accidental {
    #[default]
    Natural,

    Sharp,
    Flat,
}

impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Accidental::Natural => '♮',
            Accidental::Sharp => '♯',
            Accidental::Flat => '♭',
        };
        f.write_char(c)
    }
}

impl Display for Pitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.pitch, f)?;
        Display::fmt(&self.accidental, f)?;
        Display::fmt(&self.octave, f)
    }
}

#[derive(Error, Debug)]
#[error("invalid pitch name \"{0}\"")]
pub struct NameError(char);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_name_from_char() {
        assert!(matches!(Name::try_from('h'), Err(NameError('h'))));
        assert!(matches!(Name::try_from('a'), Ok(Name::A)));
    }

    #[test]
    fn test_pitch() {
        let note = Pitch {
            pitch: Name::A,
            accidental: Accidental::Flat,
            octave: 4,
        };
        assert_eq!(format!("{note}"), "A♭4");
    }
}
