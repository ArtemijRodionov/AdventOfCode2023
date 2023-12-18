use std::{collections::HashSet, env::args, fs};

#[derive(Default, Clone, Copy, Debug)]
struct Direction {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Direction {
    const fn opposite(&self) -> Direction {
        Direction {
            up: !self.up,
            down: !self.down,
            left: !self.left,
            right: !self.right,
        }
    }
}

#[derive(Default, Debug)]
struct Tile {
    name: u8,
    egress: Direction,
    ingress: Direction,
}

impl Tile {
    const fn new_with_same_ingress(name: u8, direction: Direction) -> Tile {
        Tile {
            name,
            egress: direction,
            ingress: direction,
        }
    }

    const fn new_with_opposite_ingress(name: u8, direction: Direction) -> Tile {
        Tile {
            name,
            egress: direction,
            ingress: direction.opposite(),
        }
    }
}

const fn get_tiles() -> [Tile; 8] {
    [
        Tile::new_with_same_ingress(
            b'S',
            Direction {
                up: true,
                down: true,
                left: true,
                right: true,
            },
        ),
        Tile::new_with_same_ingress(
            b'|',
            Direction {
                up: true,
                down: true,
                left: false,
                right: false,
            },
        ),
        Tile::new_with_same_ingress(
            b'-',
            Direction {
                up: false,
                down: false,
                left: true,
                right: true,
            },
        ),
        Tile::new_with_same_ingress(
            b'.',
            Direction {
                up: false,
                down: false,
                left: false,
                right: false,
            },
        ),
        Tile::new_with_opposite_ingress(
            b'L',
            Direction {
                up: true,
                down: false,
                left: false,
                right: true,
            },
        ),
        Tile::new_with_opposite_ingress(
            b'J',
            Direction {
                up: true,
                down: false,
                left: true,
                right: false,
            },
        ),
        Tile::new_with_opposite_ingress(
            b'7',
            Direction {
                up: false,
                down: true,
                left: true,
                right: false,
            },
        ),
        Tile::new_with_opposite_ingress(
            b'F',
            Direction {
                up: false,
                down: true,
                left: false,
                right: true,
            },
        ),
    ]
}

type TileIdx = usize;
const TILES: [Tile; 8] = get_tiles();

fn tile_idx(name: u8) -> TileIdx {
    TILES.iter().position(|v| v.name == name).unwrap()
}

fn is_up(from: TileIdx, to: TileIdx) -> bool {
    TILES[from].egress.up && TILES[to].ingress.up
}

fn is_down(from: TileIdx, to: TileIdx) -> bool {
    TILES[from].egress.down && TILES[to].ingress.down
}

fn is_left(from: TileIdx, to: TileIdx) -> bool {
    TILES[from].egress.left && TILES[to].ingress.left
}

fn is_right(from: TileIdx, to: TileIdx) -> bool {
    TILES[from].egress.right && TILES[to].ingress.right
}

fn parse_map(data: &[u8]) -> Vec<Vec<TileIdx>> {
    data.split(|c| *c == b'\n')
        .filter(|l| l.len() != 0)
        .map(|l| l.iter().map(|c| tile_idx(*c)).collect())
        .collect::<Vec<Vec<usize>>>()
}

#[derive(Default, Debug)]
struct Visitor {
    visited: HashSet<(usize, usize)>,
    path: Vec<(usize, usize)>,
}

impl Visitor {
    fn visit(&mut self, map: &Vec<Vec<TileIdx>>, i: usize, j: usize) {
        if self.visited.contains(&(i, j)) {
            return;
        }

        self.visited.insert((i, j));
        self.path.push((i, j));

        if i > 0 && is_up(map[i][j], map[i - 1][j]) {
            self.visit(map, i - 1, j);
        }
        if i < map.len() - 1 && is_down(map[i][j], map[i + 1][j]) {
            self.visit(map, i + 1, j);
        }
        if j > 0 && is_left(map[i][j], map[i][j - 1]) {
            self.visit(map, i, j - 1);
        }
        if j < map[0].len() - 1 && is_right(map[i][j], map[i][j + 1]) {
            self.visit(map, i, j + 1);
        }
    }
}

fn get_pipes(map: &Vec<Vec<TileIdx>>) -> Vec<(usize, usize)> {
    let start_tile_idx = tile_idx(b'S');

    let mut start_coord = None;
    for (i, line) in map.iter().enumerate() {
        for (j, tile_idx) in line.iter().enumerate() {
            if *tile_idx == start_tile_idx {
                start_coord = Some((i, j));
                break;
            }
        }
        if start_coord.is_some() {
            break;
        }
    }

    let (start_i, start_j) = start_coord.unwrap();
    let mut visitor = Visitor::default();
    visitor.visit(&map, start_i, start_j);
    visitor.path
}

fn solve1(data: &[u8]) -> u32 {
    let map = parse_map(data);
    let pipes = get_pipes(&map);
    (pipes.len() + 1) as u32 / 2
}

fn solve2(data: &[u8]) -> u32 {
    let map = parse_map(data);
    let pipes = get_pipes(&map);

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area = 0;
    let last = pipes[pipes.len() - 1..].iter().zip(pipes[..1].iter());
    for ((from_i, from_j), (to_i, to_j)) in pipes
        .iter()
        .take(pipes.len() - 1)
        .zip(pipes.iter().skip(1))
        .chain(last)
    {
        area += (from_i * to_j) as i32 - (from_j * to_i) as i32;
    }
    area = area.abs() / 2;

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    (area - (pipes.len() / 2 - 1) as i32) as u32
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

    // 2648 is too high
    // 394 is too high
    // 333 is not right
    // 335 is not right
    // 288 is not right
    let result2 = solve2(data);
    dbg!(result2);
}
