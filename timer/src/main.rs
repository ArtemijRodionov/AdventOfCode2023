use std::time::Instant;

fn puzzles() -> &'static [(fn(), &'static str)] {
    &[
        (day01::main, "1"),
        (day02::main, "2"),
        (day03::main, "3"),
        (day04::main, "4"),
        (day05::main, "5"),
        (day06::main, "6"),
        (day07::main, "7"),
    ]
}

pub fn main() {
    for (f, name) in puzzles() {
        let now = Instant::now();
        f();
        let elapsed = now.elapsed();
        println!("{} took {:.3}ms", name, elapsed.as_secs_f64() * 1000.0)
    }
}
