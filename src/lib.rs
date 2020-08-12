//! A no-std compatible trait for querying and manipulating bits in a
//! [`[usize]`] and iterating their run lengths.
#![no_std]

extern crate alloc;

use core::cmp::min;
use core::ops::{Bound::*, Range, RangeBounds};
use primitive_traits::Integer;

#[derive(Debug, Eq, PartialEq)]
pub struct OutOfRange();

trait RLEBits {
    fn get_bit(&self, bit: usize) -> Result<bool, OutOfRange>;
    fn set_bit(&mut self, bit: usize, value: bool) -> Result<(), OutOfRange>;
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

const WIDTH: usize = <usize as Integer>::WIDTH;

fn locate(slice: &[usize], bit: usize) -> Result<(usize, usize), OutOfRange> {
    if bit < (WIDTH * slice.len()) {
        Ok((bit / WIDTH, bit % WIDTH))
    } else {
        Err(OutOfRange())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RL {
    pub value: bool,
    pub run: Range<usize>,
}

impl RL {
    fn new(value: bool, start: usize, end: usize) -> RL {
        RL { value, run: start..end }
    }
}

pub struct RLE<'a> {
    storage: &'a [usize],
    range: Range<usize>,
    last: Option<RL>,
}

impl<'a> RLE<'a> {

    pub fn new<R: RangeBounds<usize>>(storage: &'a [usize], range: R) -> Result<RLE<'a>, OutOfRange> {
        let size = storage.len() * WIDTH;
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

    fn start_run(&self) -> Option<(usize, bool)> {
        if let Some(last) = &self.last {
            if last.run.end < self.range.end { Some((last.run.end, !last.value)) } else { None }
        } else {
            if self.range.start < self.range.end { Some((0, 1 == (self.storage[0] & 1))) } else { None }
        }
    }

    fn block(&self, block: usize, of: bool) -> usize {
        if of { !self.storage[block] } else { self.storage[block] }
    }
}

impl<'a> Iterator for RLE<'a> {
    type Item = RL;
    fn next(&mut self) -> Option<Self::Item> {
        let (start, of) = self.start_run()?;
        let x = start / WIDTH;
        let y = start % WIDTH;
        let z = WIDTH - y;
        let block = self.block(x, of) >> y;
        let len = min(block.trailing_zeros() as usize, z);
        let end = start + len;
        let ret = if end < self.range.end  {
            if len == z { // might continue into the next block!
                let extra = self.block(x+1, of).leading_zeros() as usize;
                RL::new(of, start, min(end + extra, self.range.end))
            } else {
                RL::new(of, start, end)
            }
        } else {
            RL::new(of, start, self.range.end)
        };
        self.last = Some(ret.clone());
        Some(ret)
    }
}
