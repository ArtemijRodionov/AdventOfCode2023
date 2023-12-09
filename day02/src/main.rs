use std::{cmp, env::args, fs, str::FromStr};

#[derive(Debug)]
struct ParseErr(String);

impl ParseErr {
    fn with_line(line: usize) -> Self {
        Self(format!("Can't parse at line {}", line))
    }

    fn line(&self, line: usize) -> Self {
        Self::with_line(line).msg(&self.0)
    }

    fn msg(&self, msg: &str) -> Self {
        Self(format!("{}: {}", self.0, msg))
    }
}

fn parse_num(val: &str) -> Result<u8, ParseErr> {
    u8::from_str(val.trim()).or_else(|e| {
        let msg = format!("{} for {}", e.to_string(), val);
        Err(ParseErr(msg))
    })
}

#[derive(Debug, Clone, Default)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGB {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rgb: RGB = Default::default();
        for color in s.trim().split(',') {
            let (count, name) = color
                .trim()
                .split_once(' ')
                .ok_or_else(|| ParseErr("split color and count".to_string()))?;
            let color_count = parse_num(count)?;
            match name.trim() {
                "red" => rgb.r = color_count,
                "green" => rgb.g = color_count,
                "blue" => rgb.b = color_count,
                _ => return Err(ParseErr(format!("can't match color name: {}", name))),
            };
        }

        Ok(rgb)
    }
}

#[derive(Debug)]
struct Record {
    id: u8,
    colors: Vec<RGB>,
}

#[derive(Debug)]
struct Game {
    records: Vec<Record>,
}

impl FromStr for Game {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut records = Vec::new();

        for (i, l) in s.lines().enumerate() {
            let err = ParseErr::with_line(i);
            let (game, raw_records) = l
                .split_once(":")
                .ok_or_else(|| err.msg("id and colors delimeter"))?;
            let game = game.trim();
            let id = game
                .find(' ')
                .ok_or_else(|| err.msg("find id delimeter"))
                .and_then(|idx| parse_num(&game[idx..]))?;

            let mut colors = Vec::new();
            for raw_record in raw_records.trim().split(';') {
                let color = RGB::from_str(raw_record).or_else(|err| Err(err.line(i)))?;
                colors.push(color);
            }
            records.push(Record { id, colors })
        }

        Ok(Game { records })
    }
}

impl Game {
    fn solve1(&self) -> u32 {
        let (lr, lg, lb) = (12, 13, 14);

        self.records
            .iter()
            .filter(|r| {
                for RGB { r, g, b } in r.colors.clone() {
                    if lr < r || lg < g || lb < b {
                        return false;
                    }
                }

                true
            })
            .map(|r| r.id as u32)
            .sum()
    }

    fn solve2(&self) -> u32 {
        self.records
            .iter()
            .map(|r| {
                let at_least = r.colors.iter().fold(RGB::default(), |mut i, c| {
                    i.r = cmp::max(i.r, c.r);
                    i.g = cmp::max(i.g, c.g);
                    i.b = cmp::max(i.b, c.b);

                    i
                });

                at_least.r as u32 * at_least.g as u32 * at_least.b as u32
            })
            .sum()
    }
}

fn main() {
    let path = args()
        .nth(1)
        .expect("Provide path to puzzle data as the 1th argument");
    let data = fs::read_to_string(path).expect("Can't read puzzle data");
    let game = Game::from_str(&data).expect("Can't parse puzzle");
    let result1: u32 = game.solve1();
    dbg!(result1);
    let result2: u32 = game.solve2();
    dbg!(result2);
}
