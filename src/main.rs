mod problems;

fn main() {
    let day4 = std::fs::read_to_string("inputs/day4").unwrap();
    let start = std::time::Instant::now();
    dbg!(problems::p4::solve(&day4));
    // println!(
    //     "Time: {}us",
    //     std::time::Instant::now().duration_since(start).as_micros()
    // );
}
