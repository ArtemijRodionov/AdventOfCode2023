use std::{env::args, fs};
use utils::dbg;

struct Cards([char; 13]);
const CARDS1_VALUES: Cards = Cards([
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
]);
const CARDS2_VALUES: Cards = Cards([
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
]);

impl Cards {
    fn value(&self, card: char) -> u8 {
        self.0
            .iter()
            .enumerate()
            .find(|v| *v.1 == card)
            .expect("card number")
            .0 as u8
    }
}

#[derive(Clone, Copy)]
enum HandType {
    Empty = 0,
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn hand_type_1(card_count: [u8; 13]) -> HandType {
    let mut hand_type = HandType::Empty;
    for count in card_count {
        hand_type = match (hand_type, count) {
            (HandType::Empty, 1) => HandType::HighCard,

            (HandType::Empty | HandType::HighCard, 5) => HandType::FiveKind,
            (HandType::Empty | HandType::HighCard, 4) => HandType::FourKind,
            (HandType::Empty | HandType::HighCard, 3) => HandType::ThreeKind,
            (HandType::Empty | HandType::HighCard, 2) => HandType::OnePair,

            (HandType::OnePair, 3) => HandType::FullHouse,
            (HandType::ThreeKind, 2) => HandType::FullHouse,
            (HandType::OnePair, 2) => HandType::TwoPair,

            _ => hand_type,
        };
    }
    hand_type
}

fn hand_type_2(card_count: [u8; 13]) -> HandType {
    let joker = card_count[0];
    let mut jokerless = card_count;
    jokerless[0] = 0;

    let mut hand_type = hand_type_1(jokerless);
    for _ in 0..joker {
        hand_type = match hand_type {
            HandType::Empty => HandType::HighCard,
            HandType::HighCard => HandType::OnePair,
            HandType::OnePair => HandType::ThreeKind,
            HandType::TwoPair => HandType::FullHouse,
            HandType::ThreeKind => HandType::FourKind,
            HandType::FullHouse => HandType::FourKind,
            HandType::FourKind => HandType::FiveKind,
            _ => unreachable!(),
        }
    }

    hand_type
}

trait GameRule {
    fn value(card: char) -> u8;
    fn score(count: [u8; 13]) -> u8;
}

struct Game1;
impl GameRule for Game1 {
    fn value(card: char) -> u8 {
        CARDS1_VALUES.value(card)
    }
    fn score(count: [u8; 13]) -> u8 {
        hand_type_1(count) as u8
    }
}

struct Game2;
impl GameRule for Game2 {
    fn value(card: char) -> u8 {
        CARDS2_VALUES.value(card)
    }
    fn score(count: [u8; 13]) -> u8 {
        hand_type_2(count) as u8
    }
}

fn to_hand<T: GameRule>(s: &str) -> u32 {
    if s.len() != 5 {
        unreachable!()
    }

    let mut cards_count: [u8; 13] = [0; 13];
    let hand = s
        .chars()
        .rev()
        .enumerate()
        .map(|c| {
            let card = T::value(c.1);
            cards_count[card as usize] += 1;
            (card as u32) << c.0 * 4
        })
        .fold(0, |i, v| i | v);

    let hand_type = (T::score(cards_count) as u32) << 5 * 4;

    hand_type | hand
}

fn solve<T: GameRule>(data: &str) -> u32 {
    let mut hand_bid: Vec<(u32, u32)> = data
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').expect("split");
            (to_hand::<T>(hand), bid.parse::<u32>().expect("bid"))
        })
        .collect();

    hand_bid.sort();
    hand_bid
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i as u32 + 1) * b)
        .sum()
}

pub fn main() {
    let given = args()
        .nth(1)
        .and_then(|path| fs::read_to_string(path).ok())
        .unwrap_or("".to_string());

    let builtin = include_str!("../data.txt");
    let data = if given.is_empty() { builtin } else { &given };

    let result1 = solve::<Game1>(&data);
    dbg!(result1);

    let result2 = solve::<Game2>(&data);
    dbg!(result2);
}
