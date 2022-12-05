use std::fmt::Display;

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

pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
