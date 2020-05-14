//! Key signatures
use num_derive::{FromPrimitive, ToPrimitive};

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
