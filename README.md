# Tonality

[![crate](https://img.shields.io/crates/v/tonality.svg)](https://crates.io/crates/tonality)
[![documentation](https://docs.rs/tonality/badge.svg)](https://docs.rs/tonality)
[![build](https://github.com/Stigjb/tonality/workflows/Build/badge.svg?branch=master)](https://github.com/stigjb/tonality/actions)

A library for handling tonal pitch classes, keys, intervals, accidentals and
alterations. A tonal pitch class (`Tpc`) does not distinguish pitches in
different octaves, but it does distinguish different enharmonic spellings of
notes, intervals, and keys. This is done based on the "line of fifths"
concept.

Distinguishing enharmonic spellings is desirable in several applications:

- In musical notation, where using an incorrect enharmonic spelling harms
  legibility
- When using other tunings than twelve tone equal temperament (12TET), in
  which case notes normally considered enharmonic should actually be played at
  different pitches.

Another important type is the `Step`, which represents the fact that G sharp
and G flat are written on the same line of the staff. A `Step` combined with
a `Key` or `Accidental` gives a `Tpc`.

Using the `Step` type also helps you handle octaves. If you want the F above
A flat, for instance, you would compare their `Step`s, see that F has a lower
step than A flat, and therefore should be raised an octave.

Arithmetic operations with `Tpc`s and `Interval`s return optional values,
because they may result in alterations beyond the domain of the library.
Triple sharps/flats or double diminished/augmented intervals are not
supported.

## Example

You can find the tonal pitch classes of the notes in a chord by specifying
the chord as a collection of `Interval`s.

```rust
use tonality::{Tpc, Interval};

type Chord = Vec<Interval>;

let dom7: Chord = vec![Interval::Unison, Interval::Maj3, Interval::P5, Interval::Min7];
let root = Tpc::Fs;
let chord_tones: Vec<Tpc> = dom7
    .iter()
    // `Tpc` + `Interval` returns `Option<Tpc>`
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
