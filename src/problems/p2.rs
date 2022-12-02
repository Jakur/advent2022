use super::*;

pub fn solve(input: &str) -> Answer<u32, u32> {
    let part1 = input
        .lines()
        .map(|x| {
            let mut iter = x.split_whitespace();
            let opp = parse_shape(iter.next().unwrap());
            score(opp, parse_shape(iter.next().unwrap()))
        })
        .sum::<u32>();
    let part2 = input
        .lines()
        .map(|x| {
            let mut iter = x.split_whitespace();
            let opp = parse_shape(iter.next().unwrap());
            score(opp, remap(opp, iter.next().unwrap()))
        })
        .sum::<u32>();
    Answer::new(part1, part2)
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn beats(&self) -> Self {
        match self {
            Shape::Rock => Self::Scissors,
            Shape::Paper => Self::Rock,
            Shape::Scissors => Self::Paper,
        }
    }
    fn loses(&self) -> Self {
        match self {
            Shape::Rock => Self::Paper,
            Shape::Paper => Self::Scissors,
            Shape::Scissors => Self::Rock,
        }
    }
}

fn score(opp: Shape, player: Shape) -> u32 {
    const WIN: u32 = 6;
    const DRAW: u32 = 3;
    const LOSS: u32 = 0;
    let win_score = match (opp, player) {
        (Shape::Rock, Shape::Paper) => WIN,
        (Shape::Rock, Shape::Scissors) => LOSS,
        (Shape::Paper, Shape::Rock) => LOSS,
        (Shape::Paper, Shape::Scissors) => WIN,
        (Shape::Scissors, Shape::Rock) => WIN,
        (Shape::Scissors, Shape::Paper) => LOSS,
        _ => DRAW,
    };
    win_score + player as u32
}

fn remap(opp: Shape, player: &str) -> Shape {
    match player {
        "X" => opp.beats(),
        "Y" => opp,
        _ => opp.loses(),
    }
}
fn parse_shape(c: &str) -> Shape {
    match c {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => unimplemented!(),
    }
}
