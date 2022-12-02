use crate::problems::*;

pub fn solve(input: &str) -> Answer<i32, i32> {
    let mut total = 0;
    let mut max_totals = [i32::MIN, i32::MIN, i32::MIN];
    for line in input.lines() {
        if let Ok(num) = line.parse::<i32>() {
            total += num;
        } else {
            if total > max_totals[0] {
                max_totals[0] = total;
                max_totals.sort();
            }
            total = 0;
        }
    }
    Answer::new(
        *max_totals.iter().max().unwrap(),
        max_totals.iter().sum::<i32>(),
    )
}
