use std::cmp;

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<_> = line.chars().filter_map(|char| char.to_digit(10)).collect();

            digits
                .first()
                .and_then(|first| digits.last().map(|last| first * 10 + last))
                .unwrap_or(0)
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let spelled_out_digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let regular_digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let find_indices =
        |list: [&str; 10], line: &str| -> (Option<(usize, usize)>, Option<(usize, usize)>) {
            let mut matched_digits: Vec<_> = list
                .into_iter()
                .enumerate()
                .flat_map(|(idx_digit, digit)| {
                    line.match_indices(digit)
                        .map(|(idx_line, _)| (idx_line, idx_digit))
                        .collect::<Vec<_>>()
                })
                .collect();

            matched_digits.sort_by(|(a, _), (b, _)| a.cmp(b));

            (
                matched_digits.first().copied(),
                matched_digits.last().copied(),
            )
        };

    input
        .lines()
        .map(|line| {
            let (spelled_first, spelled_last) = find_indices(spelled_out_digits, line);
            let (regular_first, regular_last) = find_indices(regular_digits, line);

            let first = match (spelled_first, regular_first) {
                (None, Some((_, idx))) => idx as u32,
                (Some((_, idx)), None) => idx as u32,
                (Some((match_spelled, idx_spelled)), Some((matched_regular, idx_regular))) => {
                    match match_spelled.cmp(&matched_regular) {
                        cmp::Ordering::Less => idx_spelled as u32,
                        cmp::Ordering::Greater => idx_regular as u32,
                        cmp::Ordering::Equal => unreachable!(),
                    }
                }
                (None, None) => unreachable!(),
            };

            let last = match (spelled_last, regular_last) {
                (None, Some((_, idx))) => idx as u32,
                (Some((_, idx)), None) => idx as u32,
                (Some((match_spelled, idx_spelled)), Some((matched_regular, idx_regular))) => {
                    match match_spelled.cmp(&matched_regular) {
                        cmp::Ordering::Less => idx_regular as u32,
                        cmp::Ordering::Greater => idx_spelled as u32,
                        cmp::Ordering::Equal => unreachable!(),
                    }
                }
                (None, None) => unreachable!(),
            };

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, part_two(test_input));
    }

    #[test]
    fn test_part2_edgecase() {
        let test_input = "gtjckhq73495fq3";

        assert_eq!(73, part_two(test_input));
    }
}
