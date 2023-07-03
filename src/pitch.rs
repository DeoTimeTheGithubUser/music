use std::fmt::{Debug, Display, Formatter, Write};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Pitch {
    pitch: Name,
    accidental: Accidental,
}

// Todo: Setters maybe
impl Pitch {
    pub fn name(&self) -> Name {
        self.pitch
    }
    pub fn accidental(&self) -> Accidental {
        self.accidental
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, strum::Display)]
pub enum Name {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
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

#[repr(i8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Accidental {
    Flat = -1,

    #[default]
    Natural = 0,

    Sharp = 1,
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
        write!(f, "{}{}", self.pitch, self.accidental)
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
}
