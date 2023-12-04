use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning_numbers, numbers_you_have) = numbers.split_once('|').unwrap();

        let winning_numbers = winning_numbers
            .split_whitespace()
            .filter_map(|d| d.trim().parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let numbers_you_have = numbers_you_have
            .split_whitespace()
            .filter_map(|d| d.trim().parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let winning_numbers_you_have = numbers_you_have.intersection(&winning_numbers).count();

        if winning_numbers_you_have > 0 {
            acc + 2_u32.pow(winning_numbers_you_have as u32 - 1)
        } else {
            acc
        }
    })
}

#[inline(always)]
fn rec_tally_copies(cards: &HashMap<usize, HashSet<usize>>, card: usize) -> u32 {
    let copies = cards.get(&card).unwrap();
    if copies.len() == 0 {
        return 1;
    }

    copies
        .iter()
        .fold(1, |acc, copy| acc + rec_tally_copies(cards, *copy))
}

fn part_two(input: &str) -> u32 {
    let mut cards: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (index, line) in input.lines().enumerate() {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning_numbers, numbers_you_have) = numbers.split_once('|').unwrap();

        let winning_numbers = winning_numbers
            .split_whitespace()
            .filter_map(|d| d.trim().parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let numbers_you_have = numbers_you_have
            .split_whitespace()
            .filter_map(|d| d.trim().parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let winning_numbers_you_have = numbers_you_have.intersection(&winning_numbers).count();

        let card_number = index + 1;
        let copies =
            ((card_number + 1)..=(winning_numbers_you_have + card_number)).collect::<HashSet<_>>();

        cards.insert(card_number, copies);
    }

    cards
        .iter()
        .fold(0, |acc, (card, _)| acc + rec_tally_copies(&cards, *card))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, part_one(input));
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(30, part_two(input));
    }
}
