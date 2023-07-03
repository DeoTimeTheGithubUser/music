use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Pitch {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Pitch {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'a' => Ok(Pitch::A),
            'b' => Ok(Pitch::B),
            'c' => Ok(Pitch::C),
            'd' => Ok(Pitch::D),
            'e' => Ok(Pitch::E),
            'f' => Ok(Pitch::F),
            'g' => Ok(Pitch::G),
            _ => Err(Error(value)),
        }
    }
}

impl Display for Pitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // debug is same as display
        Debug::fmt(self, f)
    }
}

#[derive(Error, Debug)]
#[error("invalid pitch \"{0}\"")]
pub struct Error(char);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_char() {
        assert!(matches!(Pitch::try_from('h'), Err(Error('h'))));
        assert!(matches!(Pitch::try_from('a'), Ok(Pitch::A)));
    }
}
