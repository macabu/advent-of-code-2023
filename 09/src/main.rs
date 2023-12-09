fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

#[derive(Debug)]
struct History(Vec<i64>);

impl From<&str> for History {
    fn from(line: &str) -> Self {
        let sensor_readings = line
            .split(' ')
            .flat_map(|reading| reading.parse::<i64>())
            .collect::<Vec<_>>();

        Self(sensor_readings)
    }
}

impl History {
    fn sensor_readings(&self) -> &[i64] {
        self.0.as_slice()
    }

    fn calculate_differences(&self) -> Vec<Vec<i64>> {
        let mut diff: Vec<i64> = self
            .sensor_readings()
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();

        let mut difference_history = vec![diff.clone()];

        while diff.iter().any(|x| x != &0) {
            diff = diff
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<_>>();

            difference_history.push(diff.clone());
        }

        difference_history
    }

    fn predict_next_value(&self) -> i64 {
        let difference_history = self.calculate_differences();

        let mut next_value = 0;

        for prev_history in difference_history.iter().rev() {
            let prev_value = prev_history[prev_history.len() - 1];

            next_value = prev_value + next_value;
        }

        self.sensor_readings().last().unwrap() + next_value
    }

    fn predict_previous_value(&self) -> i64 {
        let difference_history = self.calculate_differences();

        let mut next_value = 0;

        for prev_history in difference_history.iter().rev() {
            let prev_value = prev_history[0];

            next_value = prev_value - next_value;
        }

        self.sensor_readings().first().unwrap() - next_value
    }
}

fn part_one(input: &str) -> i64 {
    input
        .lines()
        .map(History::from)
        .map(|history| history.predict_next_value())
        .sum()
}

fn part_two(input: &str) -> i64 {
    input
        .lines()
        .map(History::from)
        .map(|history| history.predict_previous_value())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(18 + 28 + 68, part_one(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part_two(INPUT));
    }
}
