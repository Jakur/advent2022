mod problems;

fn main() {
    let day5 = std::fs::read_to_string("inputs/day5").unwrap();
    let start = std::time::Instant::now();
    dbg!(problems::p5::solve(&day5));
    println!(
        "Time: {}us",
        std::time::Instant::now().duration_since(start).as_micros()
    );
}
