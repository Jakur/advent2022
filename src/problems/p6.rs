use super::*;

pub fn solve(input: &str) -> Answer<usize, usize> {
    let input = input.as_bytes();
    let part1 = start_location(input, 4);
    let part2 = start_location(input, 14);
    Answer::new(part1, part2)
}

fn start_location(input: &[u8], window_size: usize) -> usize {
    for (i, window) in input.windows(window_size).enumerate() {
        let set = CharSet::from_line(window);
        if set.pop_count() as usize == window_size {
            return i + window_size;
        }
    }
    return 0;
}
