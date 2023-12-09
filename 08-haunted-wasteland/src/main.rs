use std::collections::HashMap;

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut split = input.split("\n\n");
        let instructions = split.next().unwrap();
        let nodes = split.next().unwrap();

        let instructions = instructions
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            })
            .collect::<Vec<_>>();

        let nodes = nodes
            .lines()
            .map(Node::parse)
            .map(|n| (n.name.clone(), n))
            .collect::<HashMap<String, Node>>();

        Self {
            instructions,
            nodes,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn parse(input: &str) -> Self {
        let mut name_and_connections = input.split(" = ");
        let name = name_and_connections.next().unwrap();

        let connections = name_and_connections.next().unwrap();
        let connections = connections
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap();
        let mut connections = connections.split(", ");

        let left = connections.next().unwrap();
        let right = connections.next().unwrap();

        Self {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

fn part_1(input: &str) -> usize {
    let map = Map::parse(input);

    let mut count = 0;
    let mut current_node_name = "AAA".to_string();
    let mut intruction_iter = map.instructions.iter().cycle();

    while current_node_name != "ZZZ" {
        let current_instruction = intruction_iter.next().unwrap();
        let current_node = map.nodes.get(&current_node_name).unwrap();

        let next_node_name = match current_instruction {
            Direction::Left => current_node.left.clone(),
            Direction::Right => current_node.right.clone(),
        };

        current_node_name = next_node_name;
        count += 1;
    }

    count
}

fn part_2(input: &str) -> usize {
    let map = Map::parse(input);

    let starting_positions = map
        .nodes
        .iter()
        .filter(|(_, n)| n.name.ends_with('A'))
        .map(|(_, n)| n.name.clone())
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut current_node_names = starting_positions;
    let mut intruction_iter = map.instructions.iter().cycle();

    while !current_node_names.iter().all(|n| n.ends_with('Z')) {
        let current_instruction = intruction_iter.next().unwrap();

        for current_node_name in current_node_names.iter_mut() {
            let current_node = map.nodes.get(current_node_name).unwrap();

            let next_node_name = match current_instruction {
                Direction::Left => current_node.left.clone(),
                Direction::Right => current_node.right.clone(),
            };

            *current_node_name = next_node_name;
        }

        count += 1;
    }

    count
}

fn part2_try2(input: &str) -> usize {
    let map = Map::parse(input);

    let starting_positions = map
        .nodes
        .iter()
        .filter(|(_, n)| n.name.ends_with('A'))
        .map(|(_, n)| n.name.clone())
        .collect::<Vec<_>>();

    let mut to_end_counts = HashMap::<String, usize>::new();

    for start in starting_positions.iter() {
        let mut current_node_name = start.clone();
        let mut count = 0;
        let mut intruction_iter = map.instructions.iter().cycle();

        while !current_node_name.ends_with('Z') {
            let current_instruction = intruction_iter.next().unwrap();
            let current_node = map.nodes.get(&current_node_name).unwrap();

            let next_node_name = match current_instruction {
                Direction::Left => current_node.left.clone(),
                Direction::Right => current_node.right.clone(),
            };

            current_node_name = next_node_name;
            count += 1;
        }

        to_end_counts.insert(start.clone(), count);
    }

    let counts = to_end_counts.values().cloned().collect::<Vec<_>>();

    dbg!(&counts);

    let lcm = least_common_multiple(&counts);
    dbg!(lcm);

    lcm
}

fn least_common_multiple(nums: &[usize]) -> usize {
    let mut result = 1;
    for &num in nums {
        result = num * result / gcd(num, result);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn main() {
    let sample_1_input = include_str!("sample1.input");
    let sample_1_part_1_ans = part_1(sample_1_input);
    dbg!(sample_1_part_1_ans);

    let sample_2_input = include_str!("sample2.input");
    let sample_2_part_1_ans = part_1(sample_2_input);
    dbg!(sample_2_part_1_ans);

    let my_input = include_str!("my.input");
    let my_part_1_ans = part_1(my_input);
    dbg!(my_part_1_ans);

    let sample_3_input = include_str!("sample3.input");
    let sample_3_part_2_ans = part2_try2(sample_3_input);
    dbg!(sample_3_part_2_ans);

    let my_part_2_ans = part2_try2(my_input);
    dbg!(my_part_2_ans);
}
