# rle-bitset

[![License](https://img.shields.io/crates/l/rle-bitset.svg)](https://github.com/irrustible/rle-bitset/blob/main/LICENSE)
[![Package](https://img.shields.io/crates/v/rle-bitset.svg)](https://crates.io/crates/rle-bitset)
[![Documentation](https://docs.rs/rle-bitset/badge.svg)](https://docs.rs/rle-bitset)

A no-std, no-alloc trait for querying and manipulating bits in a
[`[usize]`] and iterating their run lengths.

## Usage

```rust
use rle_bitset::*;

#[test]
fn one_two() {
    let mut x: [usize; 4] = [0, 0, 0, 0];
    let over = WORD_WIDTH * 4;
    x.set_bit(WORD_WIDTH, true).unwrap();
    assert_eq!(x.get_bit(WORD_WIDTH).unwrap(), true);
    {
        let mut iter = x.run_lengths(..).unwrap();
        assert_eq!(Some(RL::new(false, 0, WORD_WIDTH)), iter.next());
        assert_eq!(Some(RL::new(true, WORD_WIDTH, WORD_WIDTH + 1)), iter.next());
        assert_eq!(Some(RL::new(false, WORD_WIDTH + 1, over)), iter.next());
        assert_eq!(None, iter.next());
    }
    x.set_bit(WORD_WIDTH - 1, true).unwrap();
    assert_eq!(x.get_bit(WORD_WIDTH - 1).unwrap(), true);
    {
        let mut iter = x.run_lengths(..).unwrap();
        assert_eq!(Some(RL::new(false, 0, WORD_WIDTH - 1)), iter.next());
        assert_eq!(Some(RL::new(true, WORD_WIDTH -1, WORD_WIDTH + 1)), iter.next());
        assert_eq!(Some(RL::new(false, WORD_WIDTH + 1, over)), iter.next());
        assert_eq!(None, iter.next());
    }
}
```

## Copyright and License

Copyright (c) 2020 James Laver, rle-bitset contributors

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
