#![warn(clippy::pedantic)]
#![allow(dead_code)]

//! Types and operations that are useful for dealing with tonal pitch classes, which is
//! useful for music theory or musical notation.
//!
//! This library will help you answer questions like "Which accidental, if any,
//! is used for writing the pitch A flat in the key of B flat major?"
//!
//! ## Alteration versus accidental
//!
//! Though they are similar, these two types serve different purposes. An
//! Alteration can only apply to a Tpc, while an accidental can only apply to a
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
