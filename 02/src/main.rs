use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    const fn max(self) -> u32 {
        match self {
            Self::Red => 12,
            Self::Green => 13,
            Self::Blue => 14,
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => unreachable!("{value}"),
        }
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (game, reveals) = line.split_once(':').unwrap();
            let game_id = game
                .split_whitespace()
                .last()
                .and_then(|id| id.parse::<u32>().ok())
                .unwrap();

            let mut breached = false;

            'outer: for sets in reveals.split(';') {
                for set in sets.split(',') {
                    let (amount, color) = set.trim().split_once(' ').unwrap();
                    let color = Color::from(color.trim());
                    let amount = amount.trim().parse::<usize>().unwrap() as u32;

                    if amount > color.max() {
                        breached = true;

                        break 'outer;
                    }
                }
            }

            if breached {
                0
            } else {
                game_id
            }
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, reveals) = line.split_once(':').unwrap();

            let mut minimums = HashMap::with_capacity(3);

            for sets in reveals.split(';') {
                for set in sets.split(',') {
                    let (amount, color) = set.trim().split_once(' ').unwrap();
                    let color = Color::from(color.trim());
                    let amount = amount.trim().parse::<usize>().unwrap() as u32;

                    let current_min = minimums.get(&color).unwrap_or(&0);

                    if amount > *current_min {
                        minimums.insert(color, amount);
                    }
                }
            }

            minimums.get(&Color::Red).unwrap()
                * minimums.get(&Color::Green).unwrap()
                * minimums.get(&Color::Blue).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, part_one(input));
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(48 + 12 + 1560 + 630 + 36, part_two(input));
    }
}
