use crate::solution::g_seventh::HandType::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use crate::{AdventOfCode, Solution};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::panic::panic_any;

pub struct CamelCards {
    day: i32,
    hands: Vec<Hand>,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i64,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: String, bid: i64) -> Self {
        let hand_type = get_hand_type(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let card_ranking = HashMap::from([('T', 1), ('J', 2), ('Q', 3), ('K', 4), ('A', 5)]);
        if self.hand_type == other.hand_type {
            let self_cards = self.cards.chars().collect::<Vec<char>>();
            let other_cards = other.cards.chars().collect::<Vec<char>>();

            for i in 0..self_cards.len() {
                let self_char = self_cards[i];
                let other_char = other_cards[i];

                if self_char.is_ascii_digit() {
                    if other_char.is_ascii_digit() {
                        // they are both numbers
                        if self_char > other_char {
                            return Some(std::cmp::Ordering::Greater);
                        } else if self_char < other_char {
                            return Some(std::cmp::Ordering::Less);
                        }
                    } else if other_char.is_ascii_alphabetic() {
                        // self is number, other is letter, other is greater
                        return Some(std::cmp::Ordering::Less);
                    } else {
                        panic!("Invalid character");
                    }
                } else if self_char.is_ascii_alphabetic() {
                    if other_char.is_ascii_digit() {
                        // self is letter, other is number, self is greater
                        return Some(std::cmp::Ordering::Greater);
                    } else if other_char.is_ascii_alphabetic() {
                        // they are both letters
                        // T < J < Q < K < A
                        let self_rank = card_ranking.get(&self_char).unwrap();
                        let other_rank = card_ranking.get(&other_char).unwrap();
                        if self_rank > other_rank {
                            return Some(std::cmp::Ordering::Greater);
                        } else if self_rank < other_rank {
                            return Some(std::cmp::Ordering::Less);
                        }
                    } else {
                        panic!("Invalid character");
                    }
                } else {
                    panic!("Invalid character");
                }
            }
            panic!("Cannot compare hands");
        } else {
            if self.hand_type == FiveOfAKind {
                Some(std::cmp::Ordering::Greater)
            } else if other.hand_type == FiveOfAKind {
                Some(std::cmp::Ordering::Less)
            } else if self.hand_type == FourOfAKind {
                Some(std::cmp::Ordering::Greater)
            } else if other.hand_type == FourOfAKind {
                Some(std::cmp::Ordering::Less)
            } else if self.hand_type == FullHouse {
                Some(std::cmp::Ordering::Greater)
            } else if other.hand_type == FullHouse {
                Some(std::cmp::Ordering::Less)
            } else if self.hand_type == ThreeOfAKind {
                Some(std::cmp::Ordering::Greater)
            } else if other.hand_type == ThreeOfAKind {
                Some(std::cmp::Ordering::Less)
            } else if self.hand_type == TwoPair {
                Some(std::cmp::Ordering::Greater)
            } else if other.hand_type == TwoPair {
                Some(std::cmp::Ordering::Less)
            } else if self.hand_type == OnePair {
                Some(std::cmp::Ordering::Greater)
            } else if other.hand_type == OnePair {
                Some(std::cmp::Ordering::Less)
            } else {
                panic!("Invalid hand type");
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_hand_type(cards: &String) -> HandType {
    let mut map = HashMap::new();
    for card in cards.chars() {
        let count = map.entry(card).or_insert(0);
        *count += 1;
    }
    if map.values().any(|&x| x == 5) {
        return FiveOfAKind;
    } else if map.values().any(|&x| x == 4) {
        return FourOfAKind;
    } else if map.values().any(|&x| x == 3) && map.values().any(|&x| x == 2) {
        return FullHouse;
    } else if map.values().any(|&x| x == 3) {
        return ThreeOfAKind;
    } else if map.values().filter(|&x| *x == 2).count() == 2 {
        return TwoPair;
    } else if map.values().any(|&x| x == 2) {
        return OnePair;
    }
    HighCard
}

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl AdventOfCode for CamelCards {
    fn new() -> Self {
        let mut hands =
            BufReader::new(File::open("src/solution/inputs/input-07").expect("Error opening file"))
                .lines()
                .map(|line| line.unwrap())
                .map(|line| {
                    let split = line.split_whitespace().collect::<Vec<&str>>();
                    let bid = split[1].parse::<i64>().unwrap();
                    let cards = split[0];
                    let hand = Hand::new(cards.to_string(), bid);
                    println!("{:?}", hand);
                    hand
                })
                .collect::<Vec<Hand>>();
        hands.sort();
        // println!("{:?}", hands);
        CamelCards { day: 7, hands }
    }

    fn solve(&self) -> Solution {
        let part_one = self
            .hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank as i64 + 1))
            .sum::<i64>();

        Solution {
            day: self.day,
            part_one,
            part_two: 0,
        }
    }
}
