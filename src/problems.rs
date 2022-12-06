use std::fmt::Display;
use std::ops::BitAnd;

#[derive(Debug)]
pub struct Answer<T: Display, U: Display> {
    part1: T,
    part2: U,
}

impl<T: Display, U: Display> Answer<T, U> {
    fn new(part1: T, part2: U) -> Self {
        Self { part1, part2 }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct CharSet {
    data: u64,
}

impl CharSet {
    fn universal() -> Self {
        Self { data: u64::MAX }
    }
    fn from_line(cs: &[u8]) -> Self {
        let mut out = Self { data: 0 };
        for c in cs.iter().copied() {
            out.add_val(c);
        }
        out
    }
    fn add_val(&mut self, c: u8) {
        self.data |= 1 << (c - b'A');
    }
    fn find_single(&self) -> u8 {
        b'A' + self.data.trailing_zeros() as u8
    }
    fn pop_count(&self) -> u32 {
        self.data.count_ones()
    }
}

impl BitAnd for CharSet {
    type Output = CharSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        CharSet {
            data: self.data & rhs.data,
        }
    }
}

pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
