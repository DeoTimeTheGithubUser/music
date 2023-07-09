use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, strum::Display)]
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
        match value.to_ascii_uppercase() {
            'A' => Ok(Name::A),
            'B' => Ok(Name::B),
            'C' => Ok(Name::C),
            'D' => Ok(Name::D),
            'E' => Ok(Name::E),
            'F' => Ok(Name::F),
            'G' => Ok(Name::G),
            _ => Err(NameError(value)),
        }
    }
}

#[repr(i8)]
#[derive(Default, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, strum::Display)]
pub enum Accidental {
    #[strum(serialize = "♭")]
    Flat = -1,

    #[default]
    #[strum(serialize = "♮")]
    Natural = 0,

    #[strum(serialize = "♯")]
    Sharp = 1,
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
