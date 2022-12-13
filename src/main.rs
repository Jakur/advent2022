mod problems;
#[macro_use]
extern crate pest_derive;

fn main() {
    let day = 13;
    let dummy = false;
    let input = if dummy {
        std::fs::read_to_string("inputs/dummy").unwrap()
    } else {
        std::fs::read_to_string(&format!("inputs/day{}", day)).unwrap()
    };
    let start = std::time::Instant::now();
    match day {
        1 => println!("{}", problems::p1::solve(&input)),
        2 => println!("{}", problems::p2::solve(&input)),
        3 => println!("{}", problems::p3::solve(&input)),
        4 => println!("{}", problems::p4::solve(&input)),
        5 => println!("{}", problems::p5::solve(&input)),
        6 => println!("{}", problems::p6::solve(&input)),
        7 => println!("{}", problems::p7::solve(&input)),
        8 => println!("{}", problems::p8::solve(&input)),
        9 => println!("{}", problems::p9::solve(&input)),
        10 => println!("{}", problems::p10::solve(&input)),
        11 => println!("{}", problems::p11::solve(&input)),
        12 => println!("{}", problems::p12::solve(&input)),
        13 => println!("{}", problems::p13::solve(&input)),
        _ => {}
    }
    // println!("{}", problems::p12::solve(&input));
    println!(
        "Time: {}us",
        std::time::Instant::now().duration_since(start).as_micros()
    );
}
