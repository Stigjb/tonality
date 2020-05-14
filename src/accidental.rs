use num_derive::{FromPrimitive, ToPrimitive};
#[derive(Clone, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Accidental {
    DblFlat = -2,
    Flat,
    Natural,
    Sharp,
    DblSharp,
}
