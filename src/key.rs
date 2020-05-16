//! Key signatures
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::Step;
use crate::Tpc;

/// Key signatures. Named after their major key.
#[derive(Clone, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Key {
    Cb = -7,
    Gb,
    Db,
    Ab,
    Eb,
    Bb,
    F,
    C, // == 0
    G,
    D,
    A,
    E,
    B,
    Fs,
    Cs,
}

impl Default for Key {
    fn default() -> Self {
        Self::C
    }
}

impl Key {
    /// The flattest key is C flat with seven flats
    pub const MIN: Key = Key::Cb;

    /// The sharpest key is C sharp with seven sharps
    pub const MAX: Key = Key::Cs;

    /// The number of different keys
    pub const NUM_OF: isize = Self::MAX as isize - Self::MIN as isize + 1;

    /// Steps along the line of fifths to end up at an enharmonic key.
    /// Key::Gb as isize + DELTA_ENHARMONIC = Fs as isize
    pub const DELTA_ENHARMONIC: isize = 12;

    /// The root of the key's major scale
    fn root_step(&self) -> Step {
        match (self.clone() as i8).rem_euclid(7) {
            0 => Step::C,
            1 => Step::G,
            2 => Step::D,
            3 => Step::A,
            4 => Step::E,
            5 => Step::B,
            _ => Step::F,
        }
    }

    /// The root of this key's major scale
    pub fn root(&self) -> Tpc {
        FromPrimitive::from_i8(self.clone() as i8).unwrap()
    }

    /// Zero-indexed scale degrees: 0 is root, 4 is fifth
    pub fn scale_degree(&self, degree: isize) -> Tpc {
        /// Each scale degree's distance from the root, in fifths
        const OFFSETS: [i8; 7] = [0, 2, 4, -1, 1, 3, 5];
        let value = self.clone() as i8 + OFFSETS[degree.rem_euclid(7) as usize];
        FromPrimitive::from_i8(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        assert_eq!(Tpc::Cs, Key::Cs.root());
        assert_eq!(Tpc::C, Key::C.root());
    }

    #[test]
    fn test_scale_step() {
        assert_eq!(Tpc::Bb, Key::Bb.scale_degree(0));
        assert_eq!(Tpc::Es, Key::Cs.scale_degree(2));
    }
}
