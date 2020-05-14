//! Tonal pitch classes

use num_derive::{FromPrimitive, ToPrimitive};

use crate::key;
use crate::key::Key;
use crate::step::Step;
use crate::accidental::Accidental;
use crate::alteration::Alteration;

/// Tonal pitch class
///
/// Has variants for all pitch classes with double or single flats, natural,
/// double or single sharps.
///
/// Note that the "s" and "ss" suffixes mean sharp and double sharp. Should not
/// be confused with the names of flat notes, which in some languages use the -s
/// suffix.
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

/// The highest Tpc
pub const MAX: Tpc = Tpc::Bss;

/// The lowest Tpc
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

    /// The number of semitones by which the tpc is altered with respect to the key
    ///
    /// # Example
    ///
    /// ```
    /// # use tonality::{Accidental, Key, Step, Tpc};
    /// // C natural is not altered in the key of A flat
    /// assert_eq!(0, Tpc::C.alteration(Key::Ab));
    /// // A major has C sharp, so a C natural is one semitone flat
    /// assert_eq!(-1, Tpc::C.alteration(Key::A));
    /// // Db major has A flat, so an A sharp is two semitones sharp
    /// assert_eq!(2, Tpc::As.alteration(Key::Db));
    /// ```
    pub fn alteration(&self, key: Key) -> Alteration {
        let tpc = self.clone() as isize;
        let key = key.clone() as isize;
        (tpc - key - MIN as isize + key::MAX as isize) / DELTA_SEMITONE - 3
    }

    /// The accidental for the Tpc
    ///
    /// Private because you rarely want an accidental without the context of a key.
    fn accidental(&self) -> Accidental {
        match (self.clone() as isize + 1) / 7 {
            0 => Accidental::DblFlat,
            1 => Accidental::Flat,
            2 => Accidental::Natural,
            3 => Accidental::Sharp,
            _ => Accidental::DblSharp,
        }
    }

    /// Find the appropriate accidental for the Tpc in a key.
    ///
    /// If no key is given, default to C major with no fixed accidentals
    ///
    /// # Example
    /// 
    /// ```
    /// # use tonality::{Accidental, Key, Step, Tpc};
    /// let tpc = Tpc::C;
    /// let key: Option<Key> = None;
    /// assert_eq!((Step::C, None), tpc.altered_step(key));
    /// 
    /// let key: Option<Key> = Some(Key::A);
    /// assert_eq!((Step::C, Some(Accidental::Natural)), tpc.altered_step(key));
    /// 
    /// let tpc = Tpc::Fss;
    /// let key: Option<Key> = None;
    /// assert_eq!((Step::F, Some(Accidental::DblSharp)), tpc.altered_step(key));
    /// ```
    pub fn altered_step(&self, key: Option<Key>) -> (Step, Option<Accidental>) {
        let key = key.unwrap_or_default();
        let step = self.to_step();
        if step.with_key(&key) == self {
            (step, None)
        } else {
            (step, Some(self.accidental()))
        }
    }
}

/// the delta in tpc value to go 1 semitone up or down
const DELTA_SEMITONE: isize = 7;
/// the delta in tpc value to reach the next (or prev) enharmonic spelling
const DELTA_ENHARMONIC: isize = 12;

#[derive(Clone, Debug, PartialEq)]
enum Prefer {
    Flats,
    Nearest,
    Sharps,
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
        assert_eq!(0, Tpc::A.alteration(Key::C));

        // F# in C Maj: One semitone higher
        assert_eq!(1, Tpc::Fs.alteration(Key::C));

        // C in D Maj: One semitone lower
        assert_eq!(-1, Tpc::C.alteration(Key::D));

        // Fbb in C# Maj: Three semitones lower
        assert_eq!(-3, Tpc::Fbb.alteration(Key::Cs));

        // Eb in Bb Maj: No alteration
        assert_eq!(0, Tpc::Eb.alteration(Key::Bb));
    }

    #[test]
    fn test_to_accidental() {
        assert_eq!(Accidental::DblFlat, Tpc::Fbb.accidental());
        assert_eq!(Accidental::DblFlat, Tpc::Bbb.accidental());
        assert_eq!(Accidental::Flat, Tpc::Cb.accidental());
        assert_eq!(Accidental::Flat, Tpc::Eb.accidental());
        assert_eq!(Accidental::Natural, Tpc::G.accidental());
        assert_eq!(Accidental::Natural, Tpc::A.accidental());
        assert_eq!(Accidental::Sharp, Tpc::Cs.accidental());
        assert_eq!(Accidental::Sharp, Tpc::Es.accidental());
        assert_eq!(Accidental::DblSharp, Tpc::Fss.accidental());
        assert_eq!(Accidental::DblSharp, Tpc::Bss.accidental());
    }
}
