//! Key signatures
use num_derive::{FromPrimitive, ToPrimitive};

use crate::Tpc;
use crate::Step;

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

/// The flattest key is C flat with seven flats
pub const MIN: Key = Key::Cb;

/// The sharpest key is C sharp with seven sharps
pub const MAX: Key = Key::Cs;

/// The number of different keys
pub const NUM_OF: isize = MAX as isize - MIN as isize + 1;

/// Steps along the line of fifths to end up at an enharmonic key.
/// Key::Gb as isize + DELTA_ENHARMONIC = Fs as isize
pub const DELTA_ENHARMONIC: isize = 12;

impl Default for Key {
    fn default() -> Self {
        Self::C
    }
}

impl Key {
    /// The root of the key's major scale
    fn root_step(&self) -> Step {
        match self {
            Key::Cb | Key::C | Key::Cs => Step::C,
            Key::Gb | Key::G => Step::G,
            Key::Db | Key::D => Step::D,
            Key::Ab | Key::A => Step::A,
            Key::Eb | Key::E => Step::E,
            Key::Bb | Key::B => Step::B,
            Key::F | Key::Fs => Step::F,
        }
    }

    /// The root of this key's major scale
    pub fn root(&self) -> Tpc {
        self.scale_degree(0)
    }

    /// Zero-indexed scale degrees: 0 is root, 4 is fifth
    pub fn scale_degree(&self, degree: isize) -> Tpc {
        (self.root_step() + degree).with_key(self).clone()
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
