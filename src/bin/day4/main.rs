use std::cmp::min;

use lazy_static::lazy_static;
use regex::Regex;

static INPUT_FILE: &str = include_str!("./input.txt");
lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

fn parse_numbers(s: &str) -> Vec<u32> {
    NUM_REGEX
        .find_iter(s)
        .map(|num| num.as_str().parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
    copies: u32,
}

impl Card {
    fn new(s: &str) -> Self {
        let parts = s.split_once(':').unwrap().1.split_once('|').unwrap();
        Card {
            winning_numbers: parse_numbers(parts.0),
            my_numbers: parse_numbers(parts.1),
            copies: 1,
        }
    }

    fn points_total(&self) -> u32 {
        self.my_numbers.iter().fold(0, |acc, num| {
            if self.winning_numbers.contains(num) {
                if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            } else {
                acc
            }
        })
    }

    fn sum_winning(&self) -> usize {
        self.my_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count()
    }
}

fn add_cards(cards: &mut [Card]) -> u32 {
    let card = cards.first().unwrap();
    let copies = card.copies;
    let sum = min(card.sum_winning(), cards.len());
    if sum > 0 {
        cards[1..sum + 1]
            .iter_mut()
            .for_each(|c| c.copies += copies);
    };
    copies
}

fn main() {
    let mut cards: Vec<Card> = INPUT_FILE.lines().map(Card::new).collect();
    let mut total_cards = 0;

    println!(
        "Total Points: {}",
        cards
            .iter()
            .fold(0, |acc, card| { acc + card.points_total() })
    );

    for i in 0..cards.len() {
        total_cards += add_cards(&mut cards[i..]);
    }

    println!("Total scratchcards (part 2): {}", total_cards);
}
