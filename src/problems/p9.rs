use super::Answer;
use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Index {
    x: i32,
    y: i32,
}

impl Index {
    fn touching(&self, other: Index) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

#[derive(Default, Clone)]
struct Rope {
    head: Index,
    tail: Index,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Rope {
    fn update(&mut self, direction: Direction) {
        // Just naively step by 1 each time
        self.update_head(direction);
        self.update_tail();
    }
    fn update_tail(&mut self) {
        if !self.head.touching(self.tail) {
            if self.head.x != self.tail.x {
                self.tail.x += (self.head.x - self.tail.x).signum();
            }
            if self.head.y != self.tail.y {
                self.tail.y += (self.head.y - self.tail.y).signum();
            }
        }
    }
    fn update_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        }
    }
}

pub fn solve(input: &str) -> Answer<usize, usize> {
    use itertools::Itertools;
    let mut rope1 = Rope::default();
    let mut part2 = vec![Rope::default(); 9];
    let mut visited = HashSet::new();
    let mut visited2 = HashSet::new();
    visited.insert(Index::default());
    for line in input.lines() {
        let (dir, num) = line.split_whitespace().collect_tuple().unwrap();
        let dir = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unimplemented!(),
        };
        let num = num.parse().unwrap();
        // Loop part 1
        for _ in 0..num {
            rope1.update(dir);
            visited.insert(rope1.tail);
        }
        // Loop part2
        for _ in 0..num {
            part2[0].update(dir);
            for i in 1..part2.len() {
                let update_val = part2[i - 1].tail;
                let tail_rope = &mut part2[i];
                tail_rope.head = update_val;
                tail_rope.update_tail();
            }
            let real_tail = part2.last().unwrap();
            visited2.insert(real_tail.tail);
        }
    }
    Answer::new(visited.len(), visited2.len())
}
