#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct NodeRef<'a>(&'a str);

impl NodeRef<'_> {
    fn label<'a>(&'a self) -> &'a str {
        self.0
    }
}

#[derive(Debug)]
struct Node<'a> {
    source: NodeRef<'a>,
    left: NodeRef<'a>,
    right: NodeRef<'a>,
}

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn parse_input(input: &str) -> (Vec<Instruction>, Vec<Node>) {
    let instructions = input
        .lines()
        .next()
        .map(|line| {
            line.trim()
                .chars()
                .map(Instruction::from)
                .collect::<Vec<_>>()
        })
        .unwrap();

    let nodes = input
        .lines()
        .skip(2)
        .map(|line| {
            let (source, paths) = line.split_once(" = ").unwrap();

            let (path_left, path_right) = paths.split_once(", ").unwrap();
            let (path_left, path_right) = (
                path_left.trim_start_matches("("),
                path_right.trim_end_matches(")"),
            );

            Node {
                source: NodeRef(source),
                left: NodeRef(path_left),
                right: NodeRef(path_right),
            }
        })
        .collect::<Vec<_>>();

    (instructions, nodes)
}

fn part_one(input: &str) -> u32 {
    let (instructions, nodes) = parse_input(input);

    let mut current_instruction = instructions.iter().cycle();

    let mut node_index = nodes
        .iter()
        .enumerate()
        .find(|(_, node)| node.source.label() == "AAA");

    let mut visits = 0;

    while let Some((idx, _)) = node_index {
        let NodeRef(source) = nodes[idx].source;

        if source == "ZZZ" {
            break;
        }

        visits += 1;

        match current_instruction.next().unwrap() {
            Instruction::Left => {
                let NodeRef(left) = nodes[idx].left;

                if left == "ZZZ" {
                    break;
                }

                node_index = nodes
                    .iter()
                    .enumerate()
                    .find(|(_, node)| node.source.label() == left);
            }
            Instruction::Right => {
                let NodeRef(right) = nodes[idx].right;

                if right == "ZZZ" {
                    break;
                }

                node_index = nodes
                    .iter()
                    .enumerate()
                    .find(|(_, node)| node.source.label() == right);
            }
        }
    }

    visits
}

fn part_two(input: &str) -> u128 {
    let (instructions, nodes) = parse_input(input);

    let mut current_instruction = instructions.iter().cycle();

    let mut current_destinations: Vec<&Node> = nodes
        .iter()
        .filter(|node| node.source.label().ends_with("A"))
        .collect::<Vec<_>>();

    let mut visits: u128 = 0;

    while !current_destinations
        .iter()
        .all(|node| node.source.label().ends_with('Z'))
    {
        let node_indexes = current_destinations.clone();

        current_destinations.clear();

        visits += 1;

        match current_instruction.next().unwrap() {
            Instruction::Left => {
                for current_node in node_indexes.iter() {
                    let node_left = nodes
                        .iter()
                        .find(|node| node.source.label() == current_node.left.label())
                        .unwrap();

                    current_destinations.push(node_left);
                }
            }
            Instruction::Right => {
                for current_node in node_indexes.iter() {
                    let node_right = nodes
                        .iter()
                        .find(|node| node.source.label() == current_node.right.label())
                        .unwrap();

                    current_destinations.push(node_right);
                }
            }
        }
    }

    visits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let input2 = "LLR
        
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(2, part_one(input));
        assert_eq!(6, part_one(input2));
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(6, part_two(input));
    }
}
