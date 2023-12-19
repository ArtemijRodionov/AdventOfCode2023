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
        (day11::main, "11"),
    ]
}

pub fn main() {
    std::env::set_var("NODBG", "1");

    let mut total = Duration::new(0, 0);
    for (f, name) in puzzles() {
        let now = Instant::now();
        f();
        let first_elapsed = now.elapsed();

        let exps_in_sec = (1.0 / first_elapsed.as_secs_f64() - 1.0) as usize;
        let mut exps = Vec::with_capacity(exps_in_sec);
        exps.push(first_elapsed);

        for _ in 0..exps_in_sec {
            let now = Instant::now();
            f();
            exps.push(now.elapsed());
        }

        let min = exps.iter().min().unwrap().as_secs_f64();
        let max = exps.iter().max().unwrap().as_secs_f64();
        let avg = exps.iter().sum::<Duration>() / exps.len() as u32;
        let var = exps
            .iter()
            .map(|&v| (v.as_secs_f64() - avg.as_secs_f64()).powi(2))
            .sum::<f64>()
            / (exps.len() - 1) as f64;

        let std = var.sqrt();

        total += avg;
        println!(
            "{:2} Avg: {:.3}±{:.3}ms. Min: {:.3}ms. Max: {:.3}ms. Count of exps: {}",
            name,
            avg.as_secs_f64() * 1000.0,
            std * 1000.0,
            min * 1000.0,
            max * 1000.0,
            exps.len(),
        )
    }

    println!("Total {:.3}ms", total.as_secs_f64() * 1000.0)
}
