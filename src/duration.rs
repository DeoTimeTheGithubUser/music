use std::cmp::Ordering;

/// Discriminant is based on how many of this it would
/// take to make a [`Duration::Whole`]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Duration {
    Whole = 1,
    Half = 2,
    Quarter = 4,
    Eighth = 8,
    Sixteenth = 16,
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        // ordering done in reverse becaus whole > quarter etc
        (*other as u8).cmp(&(*self as u8))
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Duration {
    /// Numeric value in terms of [`Duration::Whole`]
    fn value(&self) -> f32 {
        1.0 / (*self as u8 as f32)
    }
}
