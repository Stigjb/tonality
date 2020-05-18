//! Accidentals
use num_derive::FromPrimitive;

/// Double or single flat, natural, double or single sharp
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive)]
#[allow(missing_docs)]
pub enum Accidental {
    DblFlat = -2,
    Flat,
    Natural,
    Sharp,
    DblSharp,
}
