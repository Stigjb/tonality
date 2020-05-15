# Tonality

A library for handling tonal pitch classes, keys, accidentals and
alterations.

## Example

You can find the tonal pitch classes of the notes in a chord by specifying
the chord as a collection of (scale degree, alteration) pairs.

```rust
use tonality::{Alteration, Key, Tpc};

type Chord = Vec<(isize, Alteration)>;

// The seventh in a dominant seventh chord is altered one semitone flat relative
// to the major scale
let dom7: Chord = vec![(0, 0), (2, 0), (4, 0), (6, -1)];
let key = Key::Fs;
let chord_tones: Vec<Tpc> = dom7
    .iter()
    .filter_map(|&(scale_deg, alter)| key.scale_degree(scale_deg).alter(alter))
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
