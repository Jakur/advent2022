mod problems;

fn main() {
    let day2 = std::fs::read_to_string("inputs/day2").unwrap();
    let start = std::time::Instant::now();
    dbg!(problems::p2::solve(&day2));
    // println!(
    //     "{}",
    //     std::time::Instant::now().duration_since(start).as_micros()
    // );
}
