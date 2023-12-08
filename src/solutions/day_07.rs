use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day_07.txt");
const CARD_VALUES: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: usize,
    to_value_fn: fn(char) -> usize,
}

impl Hand {
    fn to_value(&self, key: char) -> usize {
        (self.to_value_fn)(key)
    }
}

#[derive(Debug)]
struct HandParseError;

fn determine_type_with_joker(existing_type: HandType, joker_count: usize) -> HandType {
    match (existing_type, joker_count) {
        (HandType::FiveOfAKind, _) => HandType::FiveOfAKind,

        (HandType::FourOfAKind, 4) => HandType::FiveOfAKind,
        (HandType::FourOfAKind, 1) => HandType::FiveOfAKind,

        (HandType::FullHouse, 3) => HandType::FiveOfAKind,
        (HandType::FullHouse, 2) => HandType::FiveOfAKind,

        (HandType::ThreeOfAKind, 3) => HandType::FourOfAKind,
        (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,

        (HandType::TwoPair, 1) => HandType::FullHouse,
        (HandType::TwoPair, 2) => HandType::FourOfAKind,

        (HandType::OnePair, 2) => HandType::ThreeOfAKind,
        (HandType::OnePair, 1) => HandType::ThreeOfAKind,

        (HandType::HighCard, 1) => HandType::OnePair,
        other => other.0,
    }
}

fn determine_type(card_counts: HashMap<char, u32>) -> HandType {
    let mut values = card_counts.values().collect::<Vec<_>>();
    values.sort();

    match values.len() {
        1 => HandType::FiveOfAKind,
        2 => match values.as_slice() {
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            _ => panic!("Should not happen"),
        },
        3 => match values.as_slice() {
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            _ => panic!("Should not happen"),
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Should not happen"),
    }
}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(' ');
        match parts {
            Some((cards, bid)) => {
                let bid_num = bid.parse();
                let card_counts = cards.chars().fold(HashMap::new(), |mut map, key| {
                    let entry = map.entry(key).or_insert(0);
                    *entry += 1;
                    map
                });
                match bid_num {
                    Ok(num) => Ok(Hand {
                        cards: cards.chars().collect(),
                        hand_type: determine_type(card_counts),
                        bid: num,
                        to_value_fn: to_value,
                    }),
                    Err(_) => Err(HandParseError),
                }
            }
            None => Err(HandParseError),
        }
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(|h| h.parse().unwrap()).collect()
}

fn to_value(key: char) -> usize {
    CARD_VALUES.iter().position(|k| *k == key).unwrap()
}

fn to_value_joker(key: char) -> usize {
    if key == 'J' {
        return 99;
    }
    to_value(key)
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => self
                .cards
                .iter()
                .map(|n| self.to_value(*n))
                .collect::<Vec<_>>()
                .cmp(&other.cards.iter().map(|n| self.to_value(*n)).collect()),
            ord => ord,
        }
    }
}

fn determine_winnings(hands: &mut [Hand]) -> usize {
    hands.sort();
    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

fn determine_winnings_joker(hands: &[Hand]) -> usize {
    let mut hands_with_joker_value = hands
        .iter()
        .map(|hand| {
            let mut clone: Hand = hand.clone();
            clone.to_value_fn = to_value_joker;
            clone.hand_type = determine_type_with_joker(
                clone.hand_type,
                clone.cards.iter().filter(|e| **e == 'J').count(),
            );
            clone
        })
        .collect::<Vec<_>>();
    hands_with_joker_value.sort();
    hands_with_joker_value
        .iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

pub fn print_solution() {
    let mut hands = parse_input(INPUT);
    println!(
        "Camel poker winnings: {winnings}",
        winnings = determine_winnings(&mut hands)
    );
    println!(
        "Camel poker winnings with joker: {winnings}",
        winnings = determine_winnings_joker(&hands)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_determine_winnings() {
        assert_eq!(determine_winnings(&mut parse_input(SAMPLE)), 6440);
    }

    #[test]
    fn test_determine_winnings_joker() {
        assert_eq!(determine_winnings_joker(&parse_input(SAMPLE)), 5905);
    }
}
