use super::*;

pub fn solve(input: &str) -> Answer<u32, u32> {
    let mut part1 = 0;
    let mut part2 = 0;
    let lines: Vec<_> = input.lines().collect();
    for line in lines.iter() {
        part1 += line_value(line.as_bytes());
    }
    for window in lines.chunks(4) {
        let mut common = CharSet::universal();
        for line in window.iter() {
            common = common & CharSet::from_line(line.as_bytes());
        }
        part2 += priority(common.find_single());
    }
    Answer::new(part1, part2)
}

fn line_value(line: &[u8]) -> u32 {
    let (left, right) = line.split_at(line.len() / 2);
    let left_bits = CharSet::from_line(left);
    let right_bits = CharSet::from_line(right);
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
        let mut a = CharSet::default();
        let mut b = CharSet::default();
        a.add_val(b'C');
        a.add_val(b'a');
        b.add_val(b'C');
        b.add_val(b'g');
        assert_eq!((a & b).find_single(), b'C');
    }
}
