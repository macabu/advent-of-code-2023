fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

#[derive(Debug)]
struct Race {
    duration_ms: u128,
    record_distance_mm: u128,
}

impl Race {
    fn ways_to_win(&self) -> u128 {
        let mut ways_to_win = 0;

        for hold_for_ms in 0..=self.duration_ms {
            let travelled_distance_mm = hold_for_ms * (self.duration_ms - hold_for_ms);

            if travelled_distance_mm > self.record_distance_mm {
                ways_to_win += 1;
            }
        }

        ways_to_win
    }
}

fn part_one(input: &str) -> u128 {
    let (time_table, distance_table) = input.split_once('\n').unwrap();

    let times = time_table
        .trim_start_matches("Time:")
        .trim()
        .split_whitespace()
        .filter_map(|time| time.parse::<u128>().ok())
        .collect::<Vec<_>>();

    let distances = distance_table
        .trim_start_matches("Distance:")
        .trim()
        .split_whitespace()
        .filter_map(|time| time.parse::<u128>().ok())
        .collect::<Vec<_>>();

    times
        .iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            duration_ms: *time,
            record_distance_mm: distance,
        })
        .map(|race| race.ways_to_win())
        .product()
}

fn part_two(input: &str) -> u128 {
    let (time_table, distance_table) = input.split_once('\n').unwrap();

    let time = time_table
        .trim_start_matches("Time:")
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u128>()
        .unwrap();

    let distance = distance_table
        .trim_start_matches("Distance:")
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u128>()
        .unwrap();

    let race = Race {
        duration_ms: time,
        record_distance_mm: distance,
    };

    race.ways_to_win()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(4 * 8 * 9, part_one(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(71503, part_two(INPUT));
    }
}
