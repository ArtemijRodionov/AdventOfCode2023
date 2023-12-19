use std::{cmp::Ordering, env::args, fs};
use utils::dbg;

fn calc_wins(time_to_go: u64, best_distance: u64) -> u32 {
    let d = ((time_to_go.pow(2) - 4 * best_distance) as f64).sqrt();
    let x1 = (time_to_go as f64 - d) / 2.0;
    let x2 = (time_to_go as f64 + d) / 2.0;
    let (min, max) = match x1.partial_cmp(&x2) {
        Some(Ordering::Less) => (x1, x2),
        _ => (x2, x1),
    };

    (max.floor() - min.ceil()) as u32 + 1
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

pub fn main() {
    let given = args()
        .nth(1)
        .and_then(|path| fs::read_to_string(path).ok())
        .unwrap_or("".to_string());

    let builtin = include_str!("../data.txt");
    let data = if given.is_empty() { builtin } else { &given };

    let result1 = solve1(&data);
    dbg!(result1);

    let result2 = solve2(&data);
    dbg!(result2);
}
