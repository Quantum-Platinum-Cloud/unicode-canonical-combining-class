unicode-canonical-combining-class
====================

<div align="center">
  <a href="https://travis-ci.com/yeslogic/unicode-canonical-combining-class">
    <img src="https://travis-ci.com/yeslogic/unicode-canonical-combining-class.svg?branch=master" alt="Build Status"></a>
  <a href="https://docs.rs/unicode-canonical-combining-class">
    <img src="https://docs.rs/unicode-canonical-combining-class/badge.svg" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/unicode-canonical-combining-class">
    <img src="https://img.shields.io/crates/v/unicode-canonical-combining-class.svg" alt="Version">
  </a>
  <img src="https://img.shields.io/badge/unicode-15.0-informational" alt="Unicode Version">
  <a href="https://github.com/yeslogic/unicode-canonical-combining-class/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/unicode-canonical-combining-class.svg" alt="License">
  </a>
</div>

<br>

Fast lookup of the Unicode Canonical Combining Class property for `char`
in Rust using Unicode 15.0 data. This crate is no-std compatible.

Usage
-----

```rust
use unicode_canonical_combining_class::{get_canonical_combining_class, CanonicalCombiningClass};

fn main() {
    assert_eq!(get_canonical_combining_class('ཱ'), CanonicalCombiningClass::CCC129);
}
```

Performance & Implementation Notes
----------------------------------

[ucd-generate] is used to generate `tables.rs`. A build script (`build.rs`)
compiles this into a two level look up table. The look up time is constant as
it is just indexing into two arrays.

The two level approach maps a code point to a block, then to a position within
a block. This allows the second level block to be deduplicated, saving space.
The code is parameterised over the block size, which must be a power of 2. The
value in the build script is optimal for the data set.

This approach trades off some space for faster lookups. The tables take up
about 24.5KiB. Benchmarks showed this approach to be ~5–10× faster than the
typical binary search approach.

It's possible there are further optimisations that could be made to eliminate
some runs of repeated values in the first level array.

[ucd-generate]: https://github.com/yeslogic/ucd-generate
