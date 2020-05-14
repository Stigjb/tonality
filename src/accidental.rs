//! Accidentals
use num_derive::{FromPrimitive, ToPrimitive};

/// Double or single flat, natural, double or single sharp
#[derive(Clone, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Accidental {
    DblFlat = -2,
    Flat,
    Natural,
    Sharp,
    DblSharp,
}
