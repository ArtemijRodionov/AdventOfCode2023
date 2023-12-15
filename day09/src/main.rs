use std::{env::args, fs};

fn parse_line(line: &[u8]) -> Vec<i64> {
    line.split(|c| *c == b' ')
        .filter_map(|v| {
            std::str::from_utf8(v)
                .expect("from_utf8")
                .parse::<i64>()
                .ok()
        })
        .collect::<Vec<i64>>()
}

fn solve1(data: &[u8]) -> i64 {
    data.split(|c| *c == b'\n')
        .filter(|l| l != &[])
        .map(|l| {
            let mut seq = parse_line(l);
            let mut end = seq.len();

            loop {
                let mut zero = true;
                for (cur, next) in (0..end - 1).zip(1..end) {
                    seq[cur] = seq[next] - seq[cur];
                    zero = zero && seq[cur] == 0;
                }
                end -= 1;
                if zero {
                    break;
                }
            }

            seq.iter().sum::<i64>()
        })
        .sum()
}

fn solve2(data: &[u8]) -> i64 {
    data.split(|c| *c == b'\n')
        .filter(|l| l != &[])
        .map(|l| {
            let mut seq = parse_line(l);
            let mut start = 0;

            loop {
                let mut zero = true;
                for (cur, next) in (start..seq.len() - 1).zip(start + 1..seq.len()).rev() {
                    seq[next] = seq[next] - seq[cur];
                    zero = zero && seq[next] == 0;
                }
                start += 1;
                if zero {
                    break;
                }
            }

            seq.into_iter().rev().reduce(|l, r| r - l).unwrap()
        })
        .sum()
}

pub fn main() {
    let given = args()
        .nth(1)
        .and_then(|path| fs::read_to_string(path).ok())
        .unwrap_or("".to_string());

    let builtin = include_bytes!("../data.txt");
    let data = if given.is_empty() {
        builtin
    } else {
        given.as_bytes()
    };

    let result1 = solve1(data);
    dbg!(result1);

    let result2 = solve2(data);
    dbg!(result2);
}
