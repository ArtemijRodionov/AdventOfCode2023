use std::{env::args, fs};

type Node = u16;
const START: Node = 0;
const END: Node = (b'Z' - b'A') as Node;
const END_NODE: Node = END | END << 5 | END << 10;
const EMPTY_NODE: Node = Node::MAX;

fn parse_node(node: &[u8]) -> Node {
    let range = b'A'..=b'Z';
    let mut result = 0;
    for (i, v) in node.iter().rev().filter(|&v| range.contains(v)).enumerate() {
        result |= ((*v - b'A') as Node) << i * 5;
    }

    result
}

fn parse_inst<'a>(data: &'a [u8]) -> Vec<bool> {
    data.iter().map(|&x| b'L' == x).collect()
}

fn parse_map<'a>(data: impl Iterator<Item = &'a [u8]>) -> [(Node, Node); Node::MAX as usize] {
    let mut map = [(EMPTY_NODE, EMPTY_NODE); u16::MAX as usize];
    for l in data {
        let mut eq_it = l.split(|&b| b == b'=');
        let (from, to) = (eq_it.next().expect("from"), eq_it.next().expect("to"));
        let mut to_it = to.split(|&b| b == b',');
        let (to_l, to_r) = (to_it.next().expect("l"), to_it.next().expect("r"));
        map[parse_node(from) as usize] = (parse_node(to_l), parse_node(to_r));
    }
    map
}

fn solve1(data: &[u8]) -> u32 {
    let mut it = data.split(|&v| v == b'\n').filter(|v| v.len() != 0);
    let insts = parse_inst(it.next().expect("inst"));
    let map = parse_map(it);

    let mut node = START;
    for (i, inst) in insts.iter().cycle().enumerate() {
        node = if *inst {
            map[node as usize].0
        } else {
            map[node as usize].1
        };

        if node == END_NODE {
            return (i + 1) as u32;
        }
    }

    unreachable!()
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (std::cmp::max(a, b), std::cmp::min(a, b));
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn solve2(data: &[u8]) -> u64 {
    let mut it = data.split(|&v| v == b'\n').filter(|v| v.len() != 0);
    let insts = parse_inst(it.next().expect("inst"));
    let map = parse_map(it);

    let first = 0b11111 as Node;

    map.iter()
        .enumerate()
        .filter(|&v| (v.0 as Node & first) == START && *v.1 != (EMPTY_NODE, EMPTY_NODE))
        .map(|v| {
            let mut node = v.0 as Node;
            for (i, &inst) in insts.iter().cycle().enumerate() {
                node = if inst {
                    map[node as usize].0
                } else {
                    map[node as usize].1
                };
                if (node & first) == END {
                    return (i + 1) as u64;
                }
            }
            unreachable!()
        })
        .fold(1u64, lcm)
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

    // 11373818 is low
    // 12331588 is incorrect
    let result2 = solve2(data);
    dbg!(result2);
}
