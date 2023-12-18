use std::time::{Duration, Instant};

fn puzzles() -> &'static [(fn(), &'static str)] {
    &[
        (day01::main, "1"),
        (day02::main, "2"),
        (day03::main, "3"),
        (day04::main, "4"),
        // (day05::main, "5"),
        (day06::main, "6"),
        (day07::main, "7"),
        (day08::main, "8"),
        (day09::main, "9"),
        (day10::main, "10"),
    ]
}

pub fn main() {
    let mut total = Duration::new(0, 0);

    for (f, name) in puzzles() {
        let now = Instant::now();
        f();
        let elapsed = now.elapsed();
        total += elapsed;
        println!("{} took {:.3}ms\n", name, elapsed.as_secs_f64() * 1000.0)
    }
    println!("Total {:.3}ms", total.as_secs_f64() * 1000.0)
}
