use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs,
};
use utils::dbg;

fn count_wins(line: &str) -> u32 {
    let (wins, nums) = line.split_once('|').expect("|");
    let winning_nums = wins
        .split(' ')
        .filter_map(|v| v.parse::<u32>().ok())
        .collect::<HashSet<u32>>();

    nums.split(' ')
        .filter_map(|v| v.parse::<u32>().ok().and_then(|v| winning_nums.get(&v)))
        .count() as u32
}

fn solve1(data: &str) -> u32 {
    data.lines()
        .map(|line| match count_wins(line) {
            wins @ 1.. => 2u32.pow(wins - 1),
            _ => 0,
        })
        .sum()
}

fn solve2(data: &str) -> u32 {
    data.lines()
        .enumerate()
        .scan(HashMap::new(), |h, line| {
            let count = *h.entry(line.0).and_modify(|v| *v += 1).or_insert(1);
            let wins = count_wins(line.1);
            for i in 1..=wins as usize {
                let k = line.0 + i;
                h.entry(k).and_modify(|v| *v += count).or_insert(count);
            }
            h.remove(&line.0)
        })
        .sum()
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

    // low 5458681
    let result2 = solve2(&data);
    dbg!(result2);
}
