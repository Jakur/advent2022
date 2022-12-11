mod problems;

fn main() {
    let input = std::fs::read_to_string("inputs/day11").unwrap();
    let start = std::time::Instant::now();
    println!("{}", problems::p11::solve(&input));
    println!(
        "Time: {}us",
        std::time::Instant::now().duration_since(start).as_micros()
    );
}
