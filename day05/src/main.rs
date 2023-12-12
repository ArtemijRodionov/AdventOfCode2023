use rayon::prelude::*;
use std::{env::args, fs, str::Lines};

fn parse(it: &mut Lines) -> Vec<(u32, u32, u32)> {
    let mut parsed: Vec<(u32, u32, u32)> = it
        .skip_while(|&line| line.is_empty() || !line.as_bytes()[0].is_ascii_digit())
        .take_while(|&line| !line.is_empty())
        .map(|line| {
            let mut it = line
                .split(|x| x == ' ')
                .map(|x| x.parse::<u32>().expect("parse"));

            let dst = it.next().expect("1");
            let src = it.next().expect("2");
            let len = it.next().expect("3");

            (src, dst, len)
        })
        .collect();
    parsed.sort();
    parsed
}

fn parse_seed1(data: &str) -> Vec<u32> {
    data.split(' ')
        .filter_map(|v| v.parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

fn parse_seed2(data: &str) -> Vec<u32> {
    let parsed = parse_seed1(data);
    parsed
        .chunks_exact(2)
        .map(|xs| xs[0]..(xs[0] + xs[1]))
        .flatten()
        .collect()
}

fn solve(data: &str, parse_seed: fn(&str) -> Vec<u32>) -> u32 {
    let mut it = data.lines();
    let seeds = parse_seed(it.next().expect("seed"));

    let params = vec![
        parse(&mut it),
        parse(&mut it),
        parse(&mut it),
        parse(&mut it),
        parse(&mut it),
        parse(&mut it),
        parse(&mut it),
    ];

    seeds
        .par_iter()
        .map(|&seed| {
            let mut param_value = seed;
            for param in &params {
                param_value = match param.binary_search_by(|p| p.0.cmp(&param_value)) {
                    Ok(i) => param[i].1,
                    Err(i) if i == 0 => param_value,
                    Err(i) => {
                        let (src, dst, len) = param[i - 1];
                        let diff = param_value - src;
                        if len < diff {
                            param_value
                        } else {
                            dst + diff
                        }
                    }
                };
            }

            param_value
        })
        .min()
        .expect("min")
}

pub fn main() {
    let given = args()
        .nth(1)
        .and_then(|path| fs::read_to_string(path).ok())
        .unwrap_or("".to_string());

    let builtin = include_str!("../data.txt");
    let data = if given.is_empty() { builtin } else { &given };

    let result1 = solve(&data, parse_seed1);
    dbg!(result1);

    let result2 = solve(&data, parse_seed2);
    dbg!(result2);
}
