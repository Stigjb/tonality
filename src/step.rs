//! A step of a diatonic scale

use num_derive::{FromPrimitive, ToPrimitive};

use crate::accidental::Accidental;
use crate::tpc::Tpc;
use crate::key;

#[derive(Clone, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Step {
    C = 0,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl From<&Tpc> for Step {
    fn from(tpc: &Tpc) -> Self {
        match tpc {
            Tpc::Bbb | Tpc::Bb | Tpc::B | Tpc::Bs | Tpc::Bss => Step::B,
            Tpc::Fbb | Tpc::Fb | Tpc::F | Tpc::Fs | Tpc::Fss => Step::F,
            Tpc::Cbb | Tpc::Cb | Tpc::C | Tpc::Cs | Tpc::Css => Step::C,
            Tpc::Gbb | Tpc::Gb | Tpc::G | Tpc::Gs | Tpc::Gss => Step::G,
            Tpc::Dbb | Tpc::Db | Tpc::D | Tpc::Ds | Tpc::Dss => Step::D,
            Tpc::Abb | Tpc::Ab | Tpc::A | Tpc::As | Tpc::Ass => Step::A,
            Tpc::Ebb | Tpc::Eb | Tpc::E | Tpc::Es | Tpc::Ess => Step::E,
        }
    }
}

impl Step {
    /// The tonal pitch class resulting from applying an accidental to the step
    pub fn with_accidental(&self, alter: &Accidental) -> &'static Tpc {
        #[rustfmt::skip]
        const SPELLINGS: [Tpc; 35] = [
            Tpc::Cbb, Tpc::Cb, Tpc::C, Tpc::Cs, Tpc::Css,
            Tpc::Dbb, Tpc::Db, Tpc::D, Tpc::Ds, Tpc::Dss,
            Tpc::Ebb, Tpc::Eb, Tpc::E, Tpc::Es, Tpc::Ess,
            Tpc::Fbb, Tpc::Fb, Tpc::F, Tpc::Fs, Tpc::Fss,
            Tpc::Gbb, Tpc::Gb, Tpc::G, Tpc::Gs, Tpc::Gss,
            Tpc::Abb, Tpc::Ab, Tpc::A, Tpc::As, Tpc::Ass,
            Tpc::Bbb, Tpc::Bb, Tpc::B, Tpc::Bs, Tpc::Bss,
        ];
        let step = self.clone() as isize;
        let alter = alter.clone() as isize;
        let i = step * 5 + alter + 2;
        &SPELLINGS[i as usize]
    }

    /// The tonal pitch class of the step in the given key
    pub fn with_key(&self, key: &key::Key) -> &'static Tpc {
        #[rustfmt::skip]
        const BY_STEP_AND_KEY: [Tpc; 7 * key::NUM_OF as usize] = [
            Tpc::Cb, Tpc::Db, Tpc::Eb, Tpc::Fb, Tpc::Gb, Tpc::Ab, Tpc::Bb, // Cb
            Tpc::Cb, Tpc::Db, Tpc::Eb, Tpc::F,  Tpc::Gb, Tpc::Ab, Tpc::Bb, // Gb
            Tpc::C,  Tpc::Db, Tpc::Eb, Tpc::F,  Tpc::Gb, Tpc::Ab, Tpc::Bb, // Db
            Tpc::C,  Tpc::Db, Tpc::Eb, Tpc::F,  Tpc::G,  Tpc::Ab, Tpc::Bb, // Ab
            Tpc::C,  Tpc::D,  Tpc::Eb, Tpc::F,  Tpc::G,  Tpc::Ab, Tpc::Bb, // Eb
            Tpc::C,  Tpc::D,  Tpc::Eb, Tpc::F,  Tpc::G,  Tpc::A,  Tpc::Bb, // B
            Tpc::C,  Tpc::D,  Tpc::E,  Tpc::F,  Tpc::G,  Tpc::A,  Tpc::Bb, // F
            Tpc::C,  Tpc::D,  Tpc::E,  Tpc::F,  Tpc::G,  Tpc::A,  Tpc::B,  // C
            Tpc::C,  Tpc::D,  Tpc::E,  Tpc::Fs, Tpc::G,  Tpc::A,  Tpc::B,  // G
            Tpc::Cs, Tpc::D,  Tpc::E,  Tpc::Fs, Tpc::G,  Tpc::A,  Tpc::B,  // D
            Tpc::Cs, Tpc::D,  Tpc::E,  Tpc::Fs, Tpc::Gs, Tpc::A,  Tpc::B,  // A
            Tpc::Cs, Tpc::Ds, Tpc::E,  Tpc::Fs, Tpc::Gs, Tpc::A,  Tpc::B,  // E
            Tpc::Cs, Tpc::Ds, Tpc::E,  Tpc::Fs, Tpc::Gs, Tpc::As, Tpc::B,  // H
            Tpc::Cs, Tpc::Ds, Tpc::Es, Tpc::Fs, Tpc::Gs, Tpc::As, Tpc::B,  // F#
            Tpc::Cs, Tpc::Ds, Tpc::Es, Tpc::Fs, Tpc::Gs, Tpc::As, Tpc::Bs, // C#
        ];
        let key = key.clone() as isize - key::MIN as isize;
        &BY_STEP_AND_KEY[7 * key as usize + self.clone() as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_to_tpc() {
        assert_eq!(Step::C.with_accidental(&Accidental::Natural), &Tpc::C);
        assert_eq!(Step::F.with_accidental(&Accidental::Sharp), &Tpc::Fs);
        assert_eq!(Step::A.with_accidental(&Accidental::Flat), &Tpc::Ab);
        assert_eq!(Step::E.with_accidental(&Accidental::DblFlat), &Tpc::Ebb);
        assert_eq!(Step::B.with_accidental(&Accidental::Sharp), &Tpc::Bs);
    }

    #[test]
    fn test_with_key() {
        use key::Key;

        assert_eq!(&Tpc::C, Step::C.with_key(&Key::C));
        assert_eq!(&Tpc::Cs, Step::C.with_key(&Key::D));
        assert_eq!(&Tpc::B, Step::B.with_key(&Key::Fs));
        assert_eq!(&Tpc::Bb, Step::B.with_key(&Key::Ab));
        assert_eq!(&Tpc::G, Step::G.with_key(&Key::D));
        assert_eq!(&Tpc::E, Step::E.with_key(&Key::F));
    }
}
