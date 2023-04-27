# textdistance.rs

Rust library with different algorithms to compare how similar two sequences are.

Features:

+ Based on popular and battle-tested [textdistance](https://github.com/life4/textdistance) Python library (and written by the same author).
+ Zero dependency.
+ Works with any iterators, including bytes, code points, unicode grapheme clusters, words, and numbers.
+ Friendly and consistent API for all algorithms.
+ Optional normalization of the result on 0.0-1.0 interval.
+ No unsafe code.
+ Pure Rust.

## Available algorithms

Edit-based:

1. `DamerauLevenshtein`, both optimal string alignment and restricted.
1. `Hamming`
1. `Jaro`
1. `JaroWinkler`
1. `Levenshtein`
1. `Sift4`

Token-based:

1. `Jaccard` (Tanimoto)
1. `SorensenDice`
1. `Tversky`

Sequence-based:

1. `LCSSeq` (Longest Common SubSequence)
1. `LCSStr` (Longest Common SubString)
1. `RatcliffObershelp` (Gestalt pattern matching)

Normalization for other metrics:

1. `MLIPNS` normallization for `Hamming`
1. `YujianBo` normallization for `Levenshtein`

## Versioning

...

## Installation

```shell
cargo add textdistance
```

## Usage

...
