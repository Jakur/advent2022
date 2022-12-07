mod problems;

fn main() {
    let input = std::fs::read_to_string("inputs/day7").unwrap();
    let start = std::time::Instant::now();
    dbg!(problems::p7::solve(&input));
    println!(
        "Time: {}us",
        std::time::Instant::now().duration_since(start).as_micros()
    );
}
