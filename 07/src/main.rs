mod part_one;
mod part_two;

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one::part_one(input));
    dbg!(part_two::part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(
            765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5, // 6440
            part_one::part_one(INPUT)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(5905, part_two::part_two(INPUT));
    }
}
