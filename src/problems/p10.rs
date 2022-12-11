use super::Answer;

static CHECK: [i32; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Clone)]
pub struct ScanLines {
    arr: [bool; 240],
}

impl ScanLines {
    fn new() -> Self {
        Self { arr: [false; 240] }
    }
    fn mark(&mut self, idx: usize) {
        self.arr[idx] = true;
    }
    fn snazzy(&self) -> String {
        let mut s = String::new();
        for chunk in self.arr.chunks(40) {
            for x in chunk.iter() {
                if *x {
                    s.push('#');
                } else {
                    s.push('.')
                }
            }
            s.push_str("\n");
        }
        s
    }
}

impl std::fmt::Display for ScanLines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.snazzy())
    }
}

struct Machine {
    cycles: usize,
    last_reg: i32,
    reg: i32,
    total: i32,
    check_step: usize,
    scan_lines: ScanLines,
}

impl Machine {
    fn new() -> Self {
        Self {
            cycles: 1,
            last_reg: 1,
            reg: 1,
            total: 0,
            check_step: 0,
            scan_lines: ScanLines::new(),
        }
    }

    fn step(&mut self, instruction: Instruction) {
        if self.reg.abs_diff((self.cycles - 1) as i32 % 40) <= 1 {
            self.scan_lines.mark(self.cycles - 1);
        }
        match instruction {
            Instruction::NOOP => {
                self.last_reg = self.reg;
                self.update_cycles(1);
                if self.reg.abs_diff((self.cycles - 1) as i32 % 40) <= 1 {
                    self.scan_lines.mark(self.cycles - 1);
                }
            }
            Instruction::ADDX(x) => {
                self.last_reg = self.reg;
                self.reg += x;
                if self.last_reg.abs_diff((self.cycles - 1) as i32 % 40) <= 1 {
                    self.scan_lines.mark(self.cycles - 1);
                }
                if self.last_reg.abs_diff(self.cycles as i32 % 40) <= 1 {
                    self.scan_lines.mark(self.cycles);
                }
                self.update_cycles(2);
            }
        }
        if self.cycles <= 40 {
            println!("{}\n\n", &self.scan_lines.snazzy());
        }
    }
    fn update_cycles(&mut self, inc: usize) {
        let old = self.cycles;
        self.cycles += inc;
        if self.cycles % 40 == 20 {
            self.total += self.reg * CHECK[self.check_step];
            self.check_step += 1;
        } else if self.cycles % 40 > 20 && old % 40 < 20 {
            self.total += self.last_reg * CHECK[self.check_step];
            self.check_step += 1;
        }
    }
}

enum Instruction {
    NOOP,
    ADDX(i32),
}

pub fn solve(input: &str) -> Answer<i32, ScanLines> {
    // I hate this puzzle so much and its off-by-one bullshit
    let mut machine = Machine::new();
    for line in input.lines() {
        let inst = if line.starts_with("noop") {
            Instruction::NOOP
        } else {
            Instruction::ADDX(line.split_whitespace().nth(1).unwrap().parse().unwrap())
        };
        machine.step(inst);
    }
    Answer::new(machine.total, machine.scan_lines)
}
