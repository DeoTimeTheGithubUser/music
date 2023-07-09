use std::fmt::Debug;

#[repr(i8)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, strum::Display)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Dynamic {
    Pianississimo = -4,
    Pianissimo = -3,
    Piano = -2,
    #[strum(serialize = "Mezzo Piano")]
    MezzoPiano = -1,

    #[strum(serialize = "Mezzo Forte")]
    MezzoForte = 1,
    Forte = 2,
    Fortissimo = 3,
    Fortississimo = 4,
}

impl Dynamic {
    pub fn is_soft(&self) -> bool {
        (*self as i8) < 0
    }

    pub fn is_loud(&self) -> bool {
        (*self as i8) > 0
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Dynamic::Pianississimo => "ppp",
            Dynamic::Pianissimo => "pp",
            Dynamic::Piano => "p",
            Dynamic::MezzoPiano => "mp",
            Dynamic::MezzoForte => "mf",
            Dynamic::Forte => "f",
            Dynamic::Fortissimo => "ff",
            Dynamic::Fortississimo => "fff",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Dynamic::Pianississimo => "very, very soft",
            Dynamic::Pianissimo => "very soft",
            Dynamic::Piano => "soft",
            Dynamic::MezzoPiano => "medium soft",
            Dynamic::MezzoForte => "medium loud",
            Dynamic::Forte => "loud",
            Dynamic::Fortissimo => "very loud",
            Dynamic::Fortississimo => "very, very loud",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_symbol() {
        for x in Dynamic::iter() {
            if x.is_loud() {
                assert!(x.symbol().contains('f'))
            } else {
                assert!(x.symbol().contains('p'))
            }
        }
    }

    #[test]
    fn test_description() {
        for x in Dynamic::iter() {
            if x.is_loud() {
                assert!(x.description().contains("loud"))
            } else {
                assert!(x.description().contains("soft"))
            }
        }
    }
}
