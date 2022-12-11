use super::Answer;
use itertools::Itertools;
use num::Integer;

#[derive(Debug, Clone)]
struct Monkey {
    interactions: usize,
    items: Vec<u64>,
    op: Operation,
    div_test: u64,
    targets: (usize, usize),
}

impl std::default::Default for Monkey {
    fn default() -> Self {
        Self {
            interactions: Default::default(),
            items: Default::default(),
            op: Operation::Square,
            div_test: Default::default(),
            targets: Default::default(),
        }
    }
}

impl Monkey {
    fn new(items: Vec<u64>, op: Operation, div_test: u64, targets: (usize, usize)) -> Self {
        Self {
            interactions: 0,
            items,
            op,
            div_test,
            targets,
        }
    }
    fn do_turn(&mut self) -> (u64, usize) {
        self.interactions += 1;
        let item = self.items.remove(0);
        let mut item = match self.op {
            Operation::Increment(x) => item + x,
            Operation::Multiply(x) => item * x,
            Operation::Square => item * item,
        };
        item /= 3;
        if (item % self.div_test) == 0 {
            (item, self.targets.0)
        } else {
            (item, self.targets.1)
        }
    }
    fn modular_turn(&mut self, modulus: u64) -> (u64, usize) {
        self.interactions += 1;
        let item = self.items.remove(0);
        let mut item = match self.op {
            Operation::Increment(x) => item + x,
            Operation::Multiply(x) => item * x,
            Operation::Square => item * item,
        };
        item = item % modulus;
        if (item % self.div_test) == 0 {
            (item, self.targets.0)
        } else {
            (item, self.targets.1)
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Increment(u64),
    Multiply(u64),
    Square,
}

pub fn solve(input: &str) -> Answer<u64, u64> {
    let mut monkeys = Vec::new();

    let mut monkey = Monkey::default();
    for line in input.lines() {
        let line = line.trim_start();
        if line.is_empty() {
            continue;
        }
        match line.split_whitespace().next().unwrap() {
            "Starting" => {
                monkey.items = line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .filter_map(|x| x.parse().ok())
                    .collect();
            }
            "Operation:" => {
                let (num, symbol) = line
                    .split_whitespace()
                    .rev()
                    .take(2)
                    .collect_tuple()
                    .unwrap();
                dbg!(num);
                dbg!(symbol);
                if num == "old" {
                    monkey.op = Operation::Square;
                } else {
                    if symbol == "+" {
                        monkey.op = Operation::Increment(num.parse().unwrap());
                    } else {
                        monkey.op = Operation::Multiply(num.parse().unwrap());
                    }
                }
            }
            "Test:" => {
                let num = line
                    .split_whitespace()
                    .rev()
                    .next()
                    .and_then(|x| x.parse::<u64>().ok())
                    .unwrap();
                monkey.div_test = num;
            }
            "If" => {
                let target = line
                    .split_whitespace()
                    .last()
                    .and_then(|x| x.parse::<usize>().ok())
                    .unwrap();
                match line.split_whitespace().nth(1).unwrap() {
                    "true:" => {
                        monkey.targets = (target, monkey.targets.1);
                    }
                    _ => {
                        monkey.targets = (monkey.targets.0, target);
                        monkeys.push(monkey);
                        monkey = Default::default();
                    }
                }
            }
            _ => {}
        }
    }
    let mut lcm = 1;
    for monkey in monkeys.iter() {
        lcm = monkey.div_test.lcm(&lcm);
    }
    let mut part1 = monkeys.clone();
    for _ in 0..20 {
        for i in 0..part1.len() {
            while part1[i].items.len() > 0 {
                let (worry, monkey_target) = part1[i].do_turn();
                part1[monkey_target].items.push(worry);
            }
        }
    }
    const NUM_ROUNDS: usize = 10_000;
    for _ in 0..NUM_ROUNDS {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let (worry, monkey_target) = monkeys[i].modular_turn(lcm);
                monkeys[monkey_target].items.push(worry);
            }
        }
    }

    Answer::new(get_answer(&part1), get_answer(&monkeys))
}

fn get_answer(monkeys: &[Monkey]) -> u64 {
    let mut counts: Vec<_> = monkeys.iter().map(|m| m.interactions as u64).collect();
    counts.sort();
    counts.last().unwrap() * counts[counts.len() - 2]
}
