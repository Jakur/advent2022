fn main() {
    let mut total = 0;
    let mut max_totals = [i32::MIN, i32::MIN, i32::MIN];
    let day1 = std::fs::read_to_string("inputs/day1").unwrap();
    for line in day1.lines() {
        if let Ok(num) = line.parse::<i32>() {
            total += num;
        } else {
            if let Some(y) = max_totals.iter_mut().find(|x| total > **x) {
                *y = total;
                max_totals.sort();
            }
            total = 0;
        }
    }
    println!("Part1: {}", max_totals.iter().max().unwrap());
    println!("Part2: {}", max_totals.iter().sum::<i32>());
}
