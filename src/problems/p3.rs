use std::ops::BitAnd;

use super::*;

pub fn solve(input: &str) -> Answer<u32, u32> {
    let mut part1 = 0;
    let mut part2 = 0;
    let lines: Vec<_> = input.lines().collect();
    for line in lines.iter() {
        part1 += line_value(line.as_bytes());
    }
    for window in lines.chunks(3) {
        let mut common = Bits::universal();
        for line in window.iter() {
            common = common & Bits::from_line(line.as_bytes());
        }
        part2 += priority(common.find_single());
    }
    Answer::new(part1, part2)
}

#[derive(Debug, Default, Clone, Copy)]
struct Bits {
    data: u64,
}

impl Bits {
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
}

impl BitAnd for Bits {
    type Output = Bits;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bits {
            data: self.data & rhs.data,
        }
    }
}

fn line_value(line: &[u8]) -> u32 {
    let (left, right) = line.split_at(line.len() / 2);
    let left_bits = Bits::from_line(left);
    let right_bits = Bits::from_line(right);
    let common = (left_bits & right_bits).find_single();
    priority(common)
}

fn priority(val: u8) -> u32 {
    let out = if val < b'a' {
        val - b'A' + 27
    } else {
        val - b'a' + 1
    };
    out as u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut a = Bits::default();
        let mut b = Bits::default();
        a.add_val(b'C');
        a.add_val(b'a');
        b.add_val(b'C');
        b.add_val(b'g');
        assert_eq!((a & b).find_single(), b'C');
    }
}
