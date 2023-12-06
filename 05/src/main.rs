use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

#[derive(Debug)]
struct ConversionInstruction {
    destination_range_start: u128,
    source_range_start: u128,
    range_length: u128,
}

impl FromIterator<u128> for ConversionInstruction {
    fn from_iter<I: IntoIterator<Item = u128>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        Self {
            destination_range_start: iter.next().unwrap(),
            source_range_start: iter.next().unwrap(),
            range_length: iter.next().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Mapper(Vec<ConversionInstruction>);

impl Mapper {
    fn map(&self, source: u128) -> u128 {
        for instruction in self.0.iter() {
            let after_or_at_start = source >= instruction.source_range_start;
            let before_or_at_end =
                source <= instruction.source_range_start + instruction.range_length - 1;

            if after_or_at_start && before_or_at_end {
                let source_offset = source - instruction.source_range_start;

                return instruction.destination_range_start + source_offset;
            }
        }

        source
    }
}

fn parse_mappers(input: &str) -> Vec<Mapper> {
    input
        .split_terminator("\n\n")
        .skip(1)
        .map(|map| {
            let conversion_instructions = map
                .lines()
                .skip(1)
                .map(|instruction_line| {
                    instruction_line
                        .split_whitespace()
                        .filter_map(|digit| digit.parse::<u128>().ok())
                        .collect::<ConversionInstruction>()
                })
                .collect::<Vec<_>>();

            Mapper(conversion_instructions)
        })
        .collect::<Vec<_>>()
}

fn part_one(input: &str) -> u128 {
    let seeds = input
        .lines()
        .nth(0)
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .filter_map(|seed| seed.parse::<u128>().ok())
        .collect::<HashSet<_>>();

    let mappers = parse_mappers(input);

    let mut locations = vec![];

    for seed in seeds.into_iter() {
        let mut rec_seed = seed;

        for mapper in mappers.iter() {
            rec_seed = mapper.map(rec_seed);
        }

        locations.push(rec_seed);
    }

    *locations.iter().min().unwrap()
}

fn part_two(input: &str) -> u128 {
    let seed_ranges = input
        .lines()
        .nth(0)
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .filter_map(|seed| seed.parse::<u128>().ok())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();

    let mappers = parse_mappers(input);

    let mut min = u128::MAX;

    for (start, repeat) in seed_ranges {
        for i in 0..repeat {
            let mut rec_seed = start + i;

            for mapper in mappers.iter() {
                rec_seed = mapper.map(rec_seed);
            }

            if rec_seed < min {
                min = rec_seed;
            }
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(35, part_one(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part_two(INPUT));
    }
}
