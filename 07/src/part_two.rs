use std::{cmp, collections::HashMap};

#[derive(Debug, Eq, Copy, Clone, Ord, PartialEq, PartialOrd, Hash)]
enum Card {
    J = 1, // Weakest. No const generics for enums :^(
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
struct Hand(Vec<Card>);

impl From<Vec<Card>> for Hand {
    fn from(value: Vec<Card>) -> Self {
        Self(value)
    }
}

impl Ord for Hand {
    fn cmp(&self, another_hand: &Self) -> std::cmp::Ordering {
        let hand_type = self.hand_type() as u64;
        let another_hand_type = another_hand.hand_type() as u64;

        // Hands are primarily ordered based on type
        let order = hand_type.cmp(&another_hand_type);

        // If two hands have the same type, a second ordering rule takes effect.
        if order == cmp::Ordering::Equal {
            // Check each card individually by its rank to break the tie.
            for (i, card) in self.0.iter().enumerate() {
                match card.cmp(&another_hand.0[i]) {
                    cmp::Ordering::Equal => continue,
                    g @ cmp::Ordering::Greater => return g,
                    l @ cmp::Ordering::Less => return l,
                }
            }
        }

        order
    }
}

impl Hand {
    fn strongest_hand(&self) -> Option<Self> {
        let mut card_count = HashMap::new();

        for card in &self.0 {
            *card_count.entry(card).or_insert(0) += 1;
        }

        let jokers = card_count
            .iter()
            .find(|(card, _)| ***card == Card::J)
            .map(|(_, count)| count)
            .unwrap_or(&0);

        // If there are no J's, no need to make any change.
        if jokers == &0 {
            return None;
        }

        // If there are 5 J's, replace all J's with A's.
        if jokers == &5 {
            return Some(Hand(vec![Card::A, Card::A, Card::A, Card::A, Card::A]));
        }

        // Go through every card that is not J, replace all J's with that card, and calculate the type.
        let mut best_hand_candidates: HashMap<Hand, HandType> = HashMap::new();

        for (card, _) in card_count.iter().filter(|(card, _)| card != &&&Card::J) {
            let mut cloned_hand = self.0.clone();

            for i in 0..cloned_hand.len() {
                if cloned_hand[i] == Card::J {
                    cloned_hand[i] = **card;
                }
            }

            let hand = Hand(cloned_hand);
            let hand_type = hand.hand_type();

            best_hand_candidates.insert(hand, hand_type);
        }

        // Select the best hand from the candidates.
        let mut best_hand: Option<&Hand> = None;

        for (hand, _) in best_hand_candidates.iter() {
            match best_hand {
                None => {
                    best_hand = Some(hand);
                }
                Some(ref existing_best_hand) => match existing_best_hand.cmp(&hand) {
                    cmp::Ordering::Less => best_hand = Some(hand),
                    cmp::Ordering::Equal => unreachable!(),
                    cmp::Ordering::Greater => {
                        continue;
                    }
                },
            };
        }

        // unwrap: There must always be a better hand, otherwise panic.
        Some(Hand(best_hand.unwrap().0.clone()))
    }

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Bid(u64);

impl From<&str> for Bid {
    fn from(value: &str) -> Self {
        Self(value.parse::<u64>().unwrap())
    }
}

#[derive(Debug, Clone)]
struct Game(Vec<((Hand, Option<Hand>), Bid)>);

impl From<Vec<((Hand, Option<Hand>), Bid)>> for Game {
    fn from(value: Vec<((Hand, Option<Hand>), Bid)>) -> Self {
        Self(value)
    }
}

impl Game {
    fn total_winnings(&mut self) -> u64 {
        let mut cloned = self.0.clone();

        // Compare each hand using the strongest_hand, if they are equal, compare using the hand.
        cloned.sort_by(
            |((hand_a, strongest_hand_a), _), ((hand_b, strongest_hand_b), _)| {
                let a = strongest_hand_a
                    .clone()
                    .unwrap_or_else(|| hand_a.clone())
                    .hand_type();

                let b = strongest_hand_b
                    .clone()
                    .unwrap_or_else(|| hand_b.clone())
                    .hand_type();

                let hand_cmp = a.cmp(&b);

                if hand_cmp == cmp::Ordering::Equal {
                    let mut cmp = cmp::Ordering::Equal;

                    for (i, card) in hand_a.0.iter().enumerate() {
                        match card.cmp(&hand_b.0[i]) {
                            cmp::Ordering::Equal => continue,
                            other => {
                                cmp = other;

                                break;
                            }
                        }
                    }

                    cmp
                } else {
                    hand_cmp
                }
            },
        );

        cloned
            .iter()
            .enumerate()
            .fold(0, |acc, (index, (_, bid))| acc + bid.0 * (index + 1) as u64)
    }
}

pub fn part_two(input: &str) -> u64 {
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

            let strongest_hand = hand.strongest_hand();

            ((hand, strongest_hand), bid)
        })
        .collect::<Vec<((Hand, Option<Hand>), Bid)>>()
        .into();

    game.total_winnings()
}
