use std::env;
use std::fs;
use std::usize;

fn solve1(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let mut digits = line.chars().filter(|&char| char.is_ascii_digit());
            let first = digits.next().unwrap_or(' ');
            let second = digits.last().unwrap_or(first);
            let calibration = format!("{}{}", first, second);
            calibration.parse::<u32>().unwrap_or(0)
        })
        .sum()
}

const NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn find_num(line: &[u8]) -> Option<usize> {
    if line[0].is_ascii_digit() {
        return Some((line[0] - b'0') as usize);
    }

    for (i, num) in NUMBERS.iter().enumerate() {
        if line.starts_with(num) {
            return Some(i + 1);
        }
    }
    None
}

fn solve2(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let b = line.as_bytes();

            let first = (0..line.len()).find_map(|i| find_num(&b[i..])).unwrap_or(0);
            let second = (0..line.len())
                .rev()
                .find_map(|i| find_num(&b[i..]))
                .unwrap_or(0);

            first * 10 + second
        })
        .sum()
}

fn main() {
    let path = env::args().nth(1).expect("Path argument is missing");
    let data = fs::read_to_string(path).expect("File is missing");

    println!("1: {}", solve1(&data));
    println!("2: {}", solve2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve2() {
        assert_eq!(18, solve2("oneight"));
        assert_eq!(28, solve2("2oneight"));
        assert_eq!(22, solve2("2oneight2"));
        assert_eq!(
            28,
            solve2("nvvxfxbgldrb2seven7twokxzbfkvptflnhlqjrthreeoneights")
        );
    }
}
