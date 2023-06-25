use std::ops::RangeInclusive;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Tempo {
    pub bpm: u8,
}

pub trait IntoTempo {
    fn bpm(self) -> Tempo;
}

impl IntoTempo for u8 {
    fn bpm(self) -> Tempo {
        Tempo { bpm: self }
    }
}

pub type TempoKind = RangeInclusive<Tempo>;

macro_rules! tempos {
    ($($name:ident: $start:literal..=$end:literal)*) => {

        impl Tempo {
            $(
            pub const $name: TempoKind = Tempo { bpm: $start }..=Tempo { bpm: $end };
            )*
        }

    }
}

tempos! {
    LARGHISSIMO: 0..=24
    ADAGISSIMO: 24..=40
    GRAVE: 24..=40
    LARGO: 40..=66
    ADAGIO: 44..=68
    LARGHETTO: 44..=66
    ADAGIETTO: 46..=80
    LENTO: 52..=108
    ANDANTE: 56..=108
    ANDANTINO: 80..=108
    MARCIA_MODERATO: 66..=80
    ANDANTE_MODERATO: 66..=112
    MODERATO: 86..=126
    ALLEGRETTO: 76..=120
    ALLEGRO_MODERATO: 96..=120
    ALLEGRO: 100..=156
    MOLTO_ALLEGRO: 124..=160
    VIVACE: 136..=160
    VIVACISSIMO: 160..=184
    ALLEGRISSIMO: 160..=184
    PRESTO: 168..=200
    PRESTISSIMO: 200..=255
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tempo_kind() {
        assert!(Tempo::ALLEGRO.contains(&125.bpm()));
        assert_eq!(Tempo::ADAGISSIMO, Tempo::GRAVE);
    }
}
