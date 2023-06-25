use std::fmt::{Display, Formatter, Write};

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