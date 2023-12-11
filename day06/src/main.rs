use std::{env::args, fs};

fn calc_wins(time_to_go: u64, best_distance: u64) -> u32 {
    let mut wins = 0;
    for accelerate in 0..time_to_go {
        if accelerate * (time_to_go - accelerate) > best_distance {
            wins += 1;
        }
    }

    wins
}

fn parse<'a>(line: &'a str) -> impl Iterator<Item = u32> + 'a {
    line.split(' ').filter_map(|x| x.parse::<u32>().ok())
}

fn solve1(data: &str) -> u32 {
    let mut it = data.lines();
    let time = parse(it.next().expect("time"));
    let distance = parse(it.next().expect("distance"));

    time.zip(distance)
        .map(|(time_to_go, best_distance)| calc_wins(time_to_go as u64, best_distance as u64))
        .product()
}

fn parse2<'a>(line: &'a str) -> u64 {
    line.chars()
        .into_iter()
        .filter(|&v| v.is_ascii_digit())
        .collect::<String>()
        .parse()
        .expect("parse2")
}

fn solve2(data: &str) -> u32 {
    let mut it = data.lines();
    let time_to_go = parse2(it.next().expect("time"));
    let best_distance = parse2(it.next().expect("distance"));

    calc_wins(time_to_go, best_distance)
}

fn main() {
    let path = args().nth(1).expect("path");
    let data = fs::read_to_string(path).expect("data");

    let result1 = solve1(&data);
    dbg!(result1);

    let result2 = solve2(&data);
    dbg!(result2);
}
