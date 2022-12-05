use super::*;

const NUM_STACKS: usize = 9;

pub fn solve(input: &str) -> Answer<String, String> {
    let mut stacks1 = vec![Vec::new(); NUM_STACKS];
    for line in input.lines() {
        // Phase 1
        let bytes = line.as_bytes();
        if let Some(b'1') = bytes.get(1) {
            break; // End of stack diagrams
        } else {
            for (c, stack) in bytes.iter().skip(1).step_by(4).zip(stacks1.iter_mut()) {
                if *c != b' ' {
                    stack.push(*c);
                }
            }
        }
    }
    for stack in stacks1.iter_mut() {
        stack.reverse();
    }
    let mut stacks2 = stacks1.clone();
    for line in input.lines() {
        let bytes = line.as_bytes();
        // Move command
        if let Some(b'm') = bytes.get(0) {
            let (q, src, dest) = parse_line(line).unwrap();
            // Part1
            for _ in 0..q {
                let crt = stacks1[src - 1].pop().unwrap();
                stacks1[dest - 1].push(crt);
            }
            // Part2
            let len = stacks2[src - 1].len();
            let split = stacks2[src - 1].split_off(len - q);
            stacks2[dest - 1].extend_from_slice(&split);
        }
    }
    let part1: String = collect_answer(stacks1);
    let part2 = collect_answer(stacks2);
    Answer::new(part1, part2)
}

fn collect_answer(vec: Vec<Vec<u8>>) -> String {
    vec.iter()
        .filter_map(|x| x.last().map(|&x| x as char))
        .collect()
}

fn parse_line(line: &str) -> Option<(usize, usize, usize)> {
    let mut out = [0, 0, 0];
    let mut count = 0;
    for num in line.split_whitespace() {
        if let Ok(num) = num.parse() {
            out[count] = num;
            count += 1;
        }
    }
    if count == 3 {
        Some((out[0], out[1], out[2]))
    } else {
        None
    }
}
