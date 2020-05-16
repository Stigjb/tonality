# Tonality

[![crate](https://img.shields.io/crates/v/tonality.svg)](https://crates.io/crates/tonality)
[![documentation](https://docs.rs/tonality/badge.svg)](https://docs.rs/tonality)
[![build](https://github.com/Stigjb/tonality/workflows/Build/badge.svg?branch=master)](https://github.com/stigjb/tonality/actions)

A library for handling tonal pitch classes, keys, accidentals and
alterations.

## Example

You can find the tonal pitch classes of the notes in a chord by specifying
the chord as a collection of (scale degree, alteration) pairs.

```rust
use tonality::{Tpc, Interval};

type Chord = Vec<Interval>;

// The seventh in a dominant seventh chord is altered one semitone flat relative
// to the major scale
let dom7: Chord = vec![Interval::Unison, Interval::Maj3, Interval::P5, Interval::Min7];
let root = Tpc::Fs;
let chord_tones: Vec<Tpc> = dom7
    .iter()
    .filter_map(|&interval| root + interval)
    .collect();

let expected = vec![Tpc::Fs, Tpc::As, Tpc::Cs, Tpc::E];
assert_eq!(expected, chord_tones);
```

## Inspiration

Types and operations, in particular the Tpc type, are influenced by
Musescore's internal library.

## Related crates

[`pitch_calc`](https://crates.io/crates/pitch_calc) lets you convert between
a number of different pitch representations, including the frequency domain.

[`rust-music-theory`](https://crates.io/crates/rust-music-theory) supports
procedurally utilizing music theory notions like Note, Chord, Scale, Interval
and more.

This crate might support conversion to and from types in the above mentioned
crates in the future.
