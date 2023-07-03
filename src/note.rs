use crate::Pitch;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Note {
    octave: u8,
    pitch: Pitch,
}
