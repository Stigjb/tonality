//! Tonal pitch classes

use num_derive::{FromPrimitive, ToPrimitive};

use crate::key;
use crate::key::Key;
use crate::step::Step;
use crate::accidental::Accidental;
use crate::alteration::Alteration;

/// Tonal pitch class
#[derive(Clone, Debug, PartialEq, FromPrimitive, ToPrimitive)]
#[rustfmt::skip]
pub enum Tpc {
    Fbb = -1,
         Cbb, Gbb, Dbb, Abb, Ebb, Bbb,
    Fb,  Cb,  Gb,  Db,  Ab,  Eb,  Bb,
    F,   C,   G,   D,   A,   E,   B,
    Fs,  Cs,  Gs,  Ds,  As,  Es,  Bs,
    Fss, Css, Gss, Dss, Ass, Ess, Bss,
}

pub const MAX: Tpc = Tpc::Bss;
pub const MIN: Tpc = Tpc::Fbb;

impl Tpc {
    fn to_step(&self) -> Step {
        match self {
            Tpc::Bbb | Tpc::Bb | Tpc::B | Tpc::Bs | Tpc::Bss => Step::B,
            Tpc::Fbb | Tpc::Fb | Tpc::F | Tpc::Fs | Tpc::Fss => Step::F,
            Tpc::Cbb | Tpc::Cb | Tpc::C | Tpc::Cs | Tpc::Css => Step::C,
            Tpc::Gbb | Tpc::Gb | Tpc::G | Tpc::Gs | Tpc::Gss => Step::G,
            Tpc::Dbb | Tpc::Db | Tpc::D | Tpc::Ds | Tpc::Dss => Step::D,
            Tpc::Abb | Tpc::Ab | Tpc::A | Tpc::As | Tpc::Ass => Step::A,
            Tpc::Ebb | Tpc::Eb | Tpc::E | Tpc::Es | Tpc::Ess => Step::E,
        }
    }

    /// 
    pub fn to_alter_with_key(&self, key: Key) -> Alteration {
        let tpc = self.clone() as isize;
        let key = key.clone() as isize;
        (tpc - key - MIN as isize + key::MAX as isize) / DELTA_SEMITONE - 3
    }

    pub fn to_accidental(&self) -> Accidental {
        match (self.clone() as isize + 1) / 7 {
            0 => Accidental::DblFlat,
            1 => Accidental::Flat,
            2 => Accidental::Natural,
            3 => Accidental::Sharp,
            4 => Accidental::DblSharp,
            _ => unreachable!()
        }

    }

    fn to_altered_step(&self) -> (Accidental, Step) {
        todo!()
    }
}

/// the delta in tpc value to go 1 semitone up or down
const DELTA_SEMITONE: isize = 7;
/// the delta in tpc value to reach the next (or prev) enharmonic spelling
const DELTA_ENHARMONIC: isize = 12;

#[derive(Clone, Debug, PartialEq)]
pub enum Prefer {
    Flats,
    Nearest,
    Sharps,
}

pub fn pitch_to_tpc(pitch: isize, key: &Key, prefer: &Prefer) -> Tpc {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enharmonic_transpose() {
        use num_traits::ToPrimitive;
        let c = Tpc::C;
        let enharmonic = c.to_isize().unwrap() + DELTA_ENHARMONIC;
        let enharmonic: Tpc = num_traits::FromPrimitive::from_isize(enharmonic).unwrap();
        assert_eq!(enharmonic, Tpc::Bs);
    }

    #[test]
    fn test_to_alter_with_key() {
        // A in C Maj: No alteration
        assert_eq!(0, Tpc::A.to_alter_with_key(Key::C));

        // F# in C Maj: One semitone higher
        assert_eq!(1, Tpc::Fs.to_alter_with_key(Key::C));

        // C in D Maj: One semitone lower
        assert_eq!(-1, Tpc::C.to_alter_with_key(Key::D));

        // Fbb in C# Maj: Three semitones lower
        assert_eq!(-3, Tpc::Fbb.to_alter_with_key(Key::Cs));

        // Eb in Bb Maj: No alteration
        assert_eq!(0, Tpc::Eb.to_alter_with_key(Key::Bb));
    }

    #[test]
    fn test_to_accidental() {
        assert_eq!(Accidental::DblFlat, Tpc::Fbb.to_accidental());
        assert_eq!(Accidental::DblFlat, Tpc::Bbb.to_accidental());
        assert_eq!(Accidental::Flat, Tpc::Cb.to_accidental());
        assert_eq!(Accidental::Flat, Tpc::Eb.to_accidental());
        assert_eq!(Accidental::Natural, Tpc::G.to_accidental());
        assert_eq!(Accidental::Natural, Tpc::A.to_accidental());
        assert_eq!(Accidental::Sharp, Tpc::Cs.to_accidental());
        assert_eq!(Accidental::Sharp, Tpc::Es.to_accidental());
        assert_eq!(Accidental::DblSharp, Tpc::Fss.to_accidental());
        assert_eq!(Accidental::DblSharp, Tpc::Bss.to_accidental());
    }
}
