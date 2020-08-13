//! A no-std, no-alloc trait for querying and manipulating bits in a
//! [`[usize]`] and iterating their run lengths.
// #![no_std]
use core::cmp::min;
use core::ops::{Bound::*, Range, RangeBounds};
use primitive_traits::Integer;

pub const WORD_WIDTH: usize = <usize as Integer>::WIDTH;

#[derive(Debug, Eq, PartialEq)]
pub struct OutOfRange();

pub trait RLEBits {
    /// Gets the bit at the provided index, if it is within range.
    fn get_bit(&self, bit: usize) -> Result<bool, OutOfRange>;
    /// Sets the bit at the provided index, if it is within range.
    fn set_bit(&mut self, bit: usize, value: bool) -> Result<(), OutOfRange>;
    /// Returns an iterator over run lengths of the bits within the provided range.
    fn run_lengths<'a, R: RangeBounds<usize>>(&'a self, range: R) -> Result<RLE<'a>, OutOfRange>;
}

impl RLEBits for [usize] {
    fn get_bit(&self, bit: usize) ->  Result<bool, OutOfRange> {
        let (x, y) = locate(self, bit)?;
        let mask = 1 << y;
        Ok(mask == (self[x] & mask))
    }
    fn set_bit(&mut self, bit: usize, value: bool) -> Result<(), OutOfRange> {
        let (x, y) = locate(self, bit)?;
        let mask = 1 << y;
        if value {
            self[x] |= mask;
        } else {
            self[x] &= !mask;
        }
        Ok(())
    }
    fn run_lengths<'a, R: RangeBounds<usize>>(&'a self, range: R) -> Result<RLE<'a>, OutOfRange> {
        RLE::new(self, range)
    }
}

fn locate(slice: &[usize], bit: usize) -> Result<(usize, usize), OutOfRange> {
    if bit < (WORD_WIDTH * slice.len()) {
        Ok((bit / WORD_WIDTH, bit % WORD_WIDTH))
    } else {
        Err(OutOfRange())
    }
}

/// A run length - a range of the same value repeated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RL {
    pub value: bool,
    pub run: Range<usize>,
}

impl RL {
    /// Creates a new [`RL`].
    pub fn new(value: bool, start: usize, end: usize) -> RL {
        RL { value, run: start..end }
    }
}

/// An iterator over run lengths of bits.
#[derive(Debug)]
pub struct RLE<'a> {
    storage: &'a [usize],
    pub range: Range<usize>,
    last: Option<RL>,
}

impl<'a> RLE<'a> {

    pub fn new<R: RangeBounds<usize>>(storage: &'a [usize], range: R) -> Result<RLE<'a>, OutOfRange> {
        let size = storage.len() * WORD_WIDTH;
        let s = match range.start_bound() {
            Included(x) => if *x < size { Ok(*x) } else { Err(OutOfRange()) },
            Excluded(x) => if (*x+1) < size { Ok(*x+1) } else { Err(OutOfRange()) },
            Unbounded => Ok(0),
        }?;
        let e = match range.start_bound() {
            Included(x) => if *x <  size { Ok(*x+1) } else { Err(OutOfRange()) },
            Excluded(x) => if *x <= size { Ok(*x) } else { Err(OutOfRange()) },
            Unbounded => Ok(size),
        }?;
        if e >= s { Ok(RLE { storage, range: s..e, last: None }) } else { Err(OutOfRange()) }
    }

    /// If we are within the specified range, return the position to start
    fn start_run(&self) -> Option<(usize, bool)> {
        if let Some(last) = &self.last {
            if last.run.end < self.range.end { Some((last.run.end, !last.value)) } else { None }
        } else {
            if self.range.start < self.range.end { Some((self.range.start, 1 == (self.storage[0] & 1))) } else { None }
        }
    }

    // Get the current block in a manner ready for `.trailing_zeros()`.
    fn block(&self, block: usize, of: bool) -> usize {
        if of { !self.storage[block] } else { self.storage[block] }
    }
}

impl<'a> Iterator for RLE<'a> {
    type Item = RL;
    fn next(&mut self) -> Option<Self::Item> {
        let (start, of) = self.start_run()?;
        let x = start / WORD_WIDTH;
        let y = start % WORD_WIDTH;
        let bits_left = WORD_WIDTH - y; // in the block
        let block = self.block(x, of) >> y;
        let len = min(block.trailing_zeros() as usize, bits_left);
        let mut end = start + len;
        if len == bits_left {
            let mut x = x + 1;
            while end < self.range.end {
                let extra = self.block(x, of).trailing_zeros() as usize;
                end += extra;
                if extra != WORD_WIDTH { break; }
                x += 1;
            }
        }
        let ret = RL::new(of, start, min(end, self.range.end));
        self.last = Some(ret.clone());
        Some(ret)
    }
}
