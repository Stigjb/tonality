//! Intervals with enharmonic distinction
use num_derive::FromPrimitive;

/// An interval relates two tonal pitch classes to each other.
/// Note: Intervals are ordered by distance on the line of fifth, not by
/// the number of semitones.
/// ```
/// # use tonality::Interval;
/// assert!(Interval::P5 < Interval::Aug4);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Interval {
    Dim2 = -12,
    Dim6,
    Dim3,
    Dim7,
    Dim4,
    Dim1,
    Dim5,
    Min2,
    Min6,
    Min3,
    Min7,
    P4,
    Unison,
    P5,
    Maj2,
    Maj6,
    Maj3,
    Maj7,
    Aug4,
    Aug1,
    Aug5,
    Aug2,
    Aug6,
    Aug3,
    Aug7,
}

impl Interval {
    /// The biggest interval is an augmented seventh
    pub const MAX: Interval = Self::Aug7;

    /// The smallest interval is a diminished second
    pub const MIN: Interval = Self::Dim2;

    /// The number of steps along the line of fifths to an enharmonic variant
    pub const DELTA_ENHARMONIC: i8 = 12;

    /// Whether the two intervals are enharmonic, i.e. represent the same distance
    /// in semitones in twelve tone equal temperament.
    ///
    /// ```
    /// # use tonality::Interval;
    /// assert!(Interval::Aug4.enharmonic(Interval::Dim5));
    /// assert!(Interval::Aug2.enharmonic(Interval::Min3));
    /// assert!(!Interval::Unison.enharmonic(Interval::Aug1));
    /// ```
    #[must_use]
    pub fn enharmonic(self, other: Interval) -> bool {
        (self as i8 - other as i8) % Self::DELTA_ENHARMONIC == 0
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::Unison
    }
}

impl std::ops::Add<Interval> for Interval {
    type Output = Option<Self>;

    fn add(self, rhs: Interval) -> Self::Output {
        num_traits::FromPrimitive::from_i8(self as i8 + rhs as i8)
    }
}

impl std::ops::Sub<Interval> for Interval {
    type Output = Option<Self>;

    fn sub(self, rhs: Interval) -> Self::Output {
        num_traits::FromPrimitive::from_i8(self as i8 - rhs as i8)
    }
}
