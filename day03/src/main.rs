use std::{collections::HashSet, env::args, fs, u8, usize};

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}

fn get_frontier(data: &Vec<&[u8]>) -> Vec<Vec<(usize, usize)>> {
    let (r, d) = (data[0].len(), data.len());

    let mut frontier = Vec::new();
    for (i, row) in data.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if !is_symbol(col) {
                continue;
            }
            let mut parts = Vec::new();
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let (ni, nj): (usize, usize) =
                        match (((i as i32) + di).try_into(), ((j as i32) + dj).try_into()) {
                            (Ok(i), Ok(j)) => (i, j),
                            _ => continue,
                        };

                    if ni >= d || nj >= r {
                        continue;
                    }
                    if data[ni][nj].is_ascii_digit() {
                        parts.push((ni, nj));
                    }
                }
            }
            frontier.push(parts);
        }
    }

    frontier
}

#[derive(Default)]
struct Visitor {
    visited: HashSet<(usize, usize)>,
}

impl Visitor {
    fn visit(&mut self, data: &[u8], result: &mut String, i: usize, j: usize) {
        if self.visited.contains(&(i, j)) || !data[j].is_ascii_digit() {
            return;
        }

        self.visited.insert((i, j));
        if j > 0 {
            self.visit(data, result, i, j - 1);
        }
        result.push(data[j] as char);
        if j + 1 < data.len() {
            self.visit(data, result, i, j + 1);
        }
    }
}

fn solve1(data: &str) -> u32 {
    let data: Vec<&[u8]> = data.lines().map(|line| line.trim().as_bytes()).collect();

    let mut frontier = get_frontier(&data);
    let mut visitor = Visitor::default();
    let mut results = Vec::new();
    while let Some(parts) = frontier.pop() {
        for (i, j) in parts {
            let mut result = String::new();
            visitor.visit(data[i], &mut result, i, j);
            if result.len() == 0 {
                continue;
            }

            results.push(result.parse::<u32>().expect("cant parse"))
        }
    }

    results.iter().sum()
}

fn solve2(data: &str) -> u32 {
    let data: Vec<&[u8]> = data.lines().map(|line| line.trim().as_bytes()).collect();

    let mut frontier = get_frontier(&data);
    let mut visitor = Visitor::default();
    let mut results = Vec::new();
    while let Some(parts) = frontier.pop() {
        let mut parts_numbers = Vec::new();
        for (i, j) in parts {
            let mut result = String::new();
            visitor.visit(data[i], &mut result, i, j);
            if result.len() == 0 {
                continue;
            }
            let number = result.parse::<u32>().expect("cant parse");
            parts_numbers.push(number);
        }
        if parts_numbers.len() != 2 {
            continue;
        }

        results.push(parts_numbers.iter().product());
    }

    results.iter().sum()
}

fn main() {
    let path = args().nth(1).expect("path is missies");
    let data = fs::read_to_string(path).expect("can't read file");

    let result1 = solve1(&data);
    dbg!(result1);

    let result2 = solve2(&data);
    dbg!(result2);
}
