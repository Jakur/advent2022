use super::*;
use std::ops::RangeInclusive;

pub fn solve(input: &str) -> Answer<u32, u32> {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let mut iter = line.split(",");
        let mut elf1 = get_range(iter.next().unwrap()).unwrap();
        let elf2 = get_range(iter.next().unwrap()).unwrap();
        if elf1.contains(elf2.start()) && elf1.contains(elf2.end()) {
            part1 += 1;
            part2 += 1;
        } else if elf2.contains(elf1.start()) && elf2.contains(elf1.end()) {
            part1 += 1;
            part2 += 1;
        } else if elf1.any(|x| elf2.contains(&x)) {
            part2 += 1;
        }
    }
    Answer::new(part1, part2)
}

fn get_range(elf: &str) -> Option<RangeInclusive<u32>> {
    let mut iter = elf.split("-");
    let st = iter.next()?.parse().ok()?;
    let end = iter.next()?.parse().ok()?;
    Some(st..=end)
}
