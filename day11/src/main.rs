use std::{env::args, fs, vec};

fn solve(offset_value: usize, data: &[u8]) -> u64 {
    let is_new_line: fn(&u8) -> bool = |x| *x == b'\n';
    let x_count = data.iter().position(is_new_line).unwrap();
    let y_count = data.len() / x_count;

    let mut x_expand = vec![true; x_count];
    let mut y_expand = vec![true; y_count];
    let mut galaxies_count = 0;

    for (y, row) in data.split(|x| *x == b'\n').enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == b'#' {
                x_expand[x] = false;
                y_expand[y] = false;
                galaxies_count += 1;
            }
        }
    }

    let mut galaxies = Vec::new();
    // let mut galaxies = Vec::with_capacity(galaxies_count);
    let mut y_offset = 0;
    for (y, row) in data.split(is_new_line).enumerate() {
        if y_expand[y] {
            y_offset += offset_value;
        }
        let mut x_offset = 0;
        for (x, val) in row.iter().enumerate() {
            if x_expand[x] {
                x_offset += offset_value;
            }
            if *val == b'#' {
                galaxies.push(((y + y_offset) as i32, (x + x_offset) as i32));
            }
        }
    }

    // https://en.wikipedia.org/wiki/Taxicab_geometry
    let mut distance = 0;
    for (i, (from_y, from_x)) in galaxies.iter().enumerate() {
        for (to_y, to_x) in galaxies.iter().skip(i + 1) {
            distance += ((to_y - from_y).abs() + (to_x - from_x).abs()) as u64;
        }
    }

    distance
}

fn solve1(data: &[u8]) -> u32 {
    solve(1, data) as u32
}

fn solve2(data: &[u8]) -> u64 {
    solve(1000000 - 1, data)
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
