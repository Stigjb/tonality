#![warn(clippy::pedantic)]
#![allow(dead_code)]

//! Types and operations that are useful for dealing with musical notation.
//!
//! This library will help you answer questions like "Which accidental, if any,
//! is used for writing the pitch A flat in the key of B flat major?"

#[doc(inline)]
pub mod accidental;
#[doc(inline)]
pub mod alteration;
#[doc(inline)]
pub mod key;
#[doc(inline)]
pub mod pitch;
#[doc(inline)]
pub mod step;
#[doc(inline)]
pub mod tpc;
