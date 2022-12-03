mod problems;

fn main() {
    let day3 = std::fs::read_to_string("inputs/day3").unwrap();
    let start = std::time::Instant::now();
    dbg!(problems::p3::solve(&day3));
    println!(
        "Time: {}us",
        std::time::Instant::now().duration_since(start).as_micros()
    );
}
