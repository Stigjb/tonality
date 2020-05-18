#![warn(clippy::pedantic, missing_docs, missing_doc_code_examples)]
#![recursion_limit = "256"]

//! A library for handling tonal pitch classes, keys, intervals, accidentals and
//! alterations. A tonal pitch class (`Tpc`) does not distinguish pitches in
//! different octaves, but it does distinguish different enharmonic spellings of
//! notes, intervals, and keys. This is done based on the "line of fifths"
//! concept.
//! 
//! Distinguishing enharmonic spellings is desirable in several applications:
//! 
//! - In musical notation, where using an incorrect enharmonic spelling harms
//!   legibility
//! - When using other tunings than twelve tone equal temperament (12TET), in
//!   which case notes normally considered enharmonic should actually be played at
//!   different pitches.
//! 
//! Another important type is the `Step`, which represents the fact that G sharp
//! and G flat are written on the same line of the staff. A `Step` combined with
//! a `Key` or `Accidental` gives a `Tpc`.
//! 
//! Using the `Step` type also helps you handle octaves. If you want the F above
//! A flat, for instance, you would compare their `Step`s, see that F has a lower
//! step than A flat, and therefore should be raised an octave.
//! 
//! Arithmetic operations with `Tpc`s and `Interval`s return optional values,
//! because they may result in alterations beyond the domain of the library.
//! Triple sharps/flats or double diminished/augmented intervals are not
//! supported.
//!
//! ## Alteration versus accidental
//!
//! Though they are similar, these two types serve different purposes. An
//! `Alteration` is a relative change that applies to a `Tpc`.
//! An accidental is an absolute change that can only apply to a
//! Step - turning it into a Tpc.
//!
//! # Example
//!
//! It can be used for finding the tonal pitch classes in a chord:
//!
//! ```
//! # use tonality::*;
//! let root = Tpc::Fs;
//! type Chord = Vec<Interval>;
//! let dom7: Chord = {
//!     use Interval::*;
//!     vec![Unison, Maj3, P5, Min7]
//! };
//! let tpcs: Vec<Tpc> = dom7
//!     .iter()
//!     .filter_map(|&interval| root + interval)
//!     .collect();
//! let expected = vec![Tpc::Fs, Tpc::As, Tpc::Cs, Tpc::E];
//! assert_eq!(expected, tpcs);
//! ```

#[doc(inline)]
pub mod accidental;
#[doc(inline)]
pub mod alteration;
#[doc(inline)]
pub mod interval;
#[doc(inline)]
pub mod key;
#[doc(inline)]
pub mod step;
#[doc(inline)]
pub mod tpc;

pub use {
    accidental::Accidental, alteration::Alteration, interval::Interval, key::Key, step::Step,
    tpc::Tpc,
};
