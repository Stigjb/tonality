//! Tonal pitch classes
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{Accidental, Alteration, Interval, Key, Step};

/// Tonal pitch class
///
/// Has variants for all pitch classes with double or single flats, natural,
/// double or single sharps. The numeric value of the enum corresponds to steps
/// away from C on the ``line of fifths''. For instance, F is one fifth below C
/// and has the value -1.
///
/// Note that the "s" and "ss" suffixes mean sharp and double sharp. Should not
/// be confused with the names of flat notes, which in some languages use the -s
/// suffix.
#[derive(Clone, Copy, PartialOrd, Ord, Eq, Debug, PartialEq, FromPrimitive)]
#[must_use]
#[rustfmt::skip]
pub enum Tpc {
    Fbb = -15,
         Cbb, Gbb, Dbb, Abb, Ebb, Bbb,
    Fb,  Cb,  Gb,  Db,  Ab,  Eb,  Bb,
    F,   C,   G,   D,   A,   E,   B,
    Fs,  Cs,  Gs,  Ds,  As,  Es,  Bs,
    Fss, Css, Gss, Dss, Ass, Ess, Bss,
}

impl Tpc {
    /// The sharpest valid Tpc: B double sharp
    pub const MAX: Tpc = Tpc::Bss;

    /// The flattest valid Tpc: F double flat
    pub const MIN: Tpc = Tpc::Fbb;

    /// Number of fifths to add to be a semitone higher
    const DELTA_SEMITONE: i8 = 7;

    /// Number of fifths to the next enharmonic spelling
    const DELTA_ENHARMONIC: i8 = 12;

    /// The basic step of the Tpc, or where it is placed on the staff
    pub fn step(self) -> Step {
        match (self as i8).rem_euclid(7) {
            0 => Step::C,
            1 => Step::G,
            2 => Step::D,
            3 => Step::A,
            4 => Step::E,
            5 => Step::B,
            _ => Step::F,
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
    #[must_use]
    pub fn alteration(self, key: Key) -> Alteration {
        let tpc = self as i8;
        let key = key as i8;
        (tpc - key - Self::MIN as i8 + Key::MAX as i8) / Self::DELTA_SEMITONE - 3
    }

    /// The accidental for the Tpc
    ///
    /// Private because you rarely want an accidental without the context of a key.
    fn accidental(self) -> Accidental {
        match (self as i8 + 1).div_euclid(7) {
            -2 => Accidental::DblFlat,
            -1 => Accidental::Flat,
            0 => Accidental::Natural,
            1 => Accidental::Sharp,
            2 => Accidental::DblSharp,
            _ => unreachable!("Tpc out of range"),
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
    pub fn altered_step(self, key: Option<Key>) -> (Step, Option<Accidental>) {
        let key = key.unwrap_or_default();
        let step = self.step();
        if step.with_key(key) == self {
            (step, None)
        } else {
            (step, Some(self.accidental()))
        }
    }

    /// Adjust alteration while maintaining the step value
    ///
    /// Returns None if the alteration would be sharper than double sharp or
    /// flatter than double flat
    #[must_use]
    pub fn alter(self, by: Alteration) -> Option<Tpc> {
        let new = self as i8 + by * Self::DELTA_SEMITONE;
        num_traits::FromPrimitive::from_i8(new)
    }
}

impl std::ops::Add<Interval> for Tpc {
    type Output = Option<Tpc>;

    fn add(self, rhs: Interval) -> Self::Output {
        let value = self as i8 + rhs as i8;
        FromPrimitive::from_i8(value)
    }
}

impl std::ops::Sub<Interval> for Tpc {
    type Output = Option<Tpc>;

    fn sub(self, rhs: Interval) -> Self::Output {
        let value = self as i8 - rhs as i8;
        FromPrimitive::from_i8(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enharmonic_transpose() {
        let c = Tpc::C;
        let enharmonic = c as i8 + Tpc::DELTA_ENHARMONIC;
        let enharmonic: Tpc = num_traits::FromPrimitive::from_i8(enharmonic).unwrap();
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

    fn property_alter_keeps_step(tpc: Tpc, alter: Alteration) {
        if let Some(altered) = tpc.alter(alter) {
            assert_eq!(tpc.step(), altered.step())
        }
    }

    #[test]
    fn test_alter_property() {
        for tpc in vec![Tpc::C, Tpc::Ab, Tpc::E, Tpc::Fss, Tpc::Gb] {
            for alter in vec![0, 1, 2, -3, -2] {
                property_alter_keeps_step(tpc, alter)
            }
        }
    }

    #[test]
    fn add_interval() {
        assert_eq!(Some(Tpc::E), Tpc::C + Interval::Maj3);
        assert_eq!(Some(Tpc::C), Tpc::Fs + Interval::Dim5);

        // A dim 5th from Fbb would be Cbbb - out of range
        assert_eq!(None, Tpc::Fbb + Interval::Dim5);

        // A major 3rd above D## would be F### - out of range
        assert_eq!(None, Tpc::Dss + Interval::Maj3);
    }
}
