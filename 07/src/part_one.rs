use std::{cmp, collections::HashMap};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand(Vec<Card>);

impl From<Vec<Card>> for Hand {
    fn from(value: Vec<Card>) -> Self {
        Self(value)
    }
}

impl Ord for Hand {
    fn cmp(&self, another_hand: &Self) -> std::cmp::Ordering {
        let hand_type = self.hand_type() as u32;
        let another_hand_type = another_hand.hand_type() as u32;

        // Hands are primarily ordered based on type
        let order = hand_type.cmp(&another_hand_type);

        // If two hands have the same type, a second ordering rule takes effect.
        if order == cmp::Ordering::Equal {
            // Check each card individually by its rank to break the tie.
            for (i, card) in self.0.iter().enumerate() {
                match card.cmp(&another_hand.0[i]) {
                    cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }
        }

        order
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_count = HashMap::new();

        for card in &self.0 {
            *card_count.entry(card).or_insert(0) += 1;
        }

        // 1. Five of a kind, where all five cards have the same label: AAAAA
        if card_count.iter().any(|(_, count)| *count == 5) {
            return HandType::FiveOfAKind;
        }

        // 2. Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        if card_count.iter().any(|(_, count)| *count == 4) {
            return HandType::FourOfAKind;
        }

        // 3. Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        if card_count.iter().any(|(_, count)| *count == 3) {
            if card_count.len() == 2 {
                return HandType::FullHouse;
            }

            // 4. Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
            return HandType::ThreeOfAKind;
        }

        // 5. Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        if card_count.iter().any(|(_, count)| *count == 2) {
            if card_count.len() == 3 {
                return HandType::TwoPair;
            }

            // 6. One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
            return HandType::OnePair;
        }

        // 7. High card, where all cards' labels are distinct: 23456
        HandType::HighCard
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Bid(u32);

impl From<&str> for Bid {
    fn from(value: &str) -> Self {
        Self(value.parse::<u32>().unwrap())
    }
}

#[derive(Debug)]
struct Game(Vec<(Hand, Bid)>);

impl From<Vec<(Hand, Bid)>> for Game {
    fn from(value: Vec<(Hand, Bid)>) -> Self {
        Self(value)
    }
}

impl Game {
    fn total_winnings(&mut self) -> u32 {
        self.0
            .sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

        self.0
            .iter()
            .enumerate()
            .fold(0, |acc, (index, (_, bid))| acc + bid.0 * (index + 1) as u32)
    }
}

pub fn part_one(input: &str) -> u32 {
    let mut game: Game = input
        .lines()
        .map(|line| {
            let (hand_line, bid) = line.split_once(' ').unwrap();

            let hand: Hand = hand_line
                .chars()
                .map(Card::from)
                .collect::<Vec<Card>>()
                .into();

            let bid: Bid = bid.into();

            (hand, bid)
        })
        .collect::<Vec<(Hand, Bid)>>()
        .into();

    game.total_winnings()
}
