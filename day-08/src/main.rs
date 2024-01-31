use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Instruction::Left),
            "R" => Ok(Instruction::Right),
            _ => Err(ParseInstructionError),
        }
    }
}

#[derive(Clone, Debug)]
struct NodeElements(String, String);

fn main() {
    println!("Part One Ex A (2): {}", part_one("example-1a.txt"));
    println!("Part One Ex B (6): {}", part_one("example-1b.txt"));
    println!("Part One Input: {}", part_one("input.txt"));

    println!("Part Two Ex (6): {}", part_two("example-2.txt"));
    println!("Part Two Input: {}", part_two("input.txt"));
}

fn parse_input(input: &str) -> (Vec<Instruction>, HashMap<String, NodeElements>) {
    let mut lines = input.lines();
    let instructions: Vec<Instruction> = lines
        .next()
        .expect("Could not find instructions")
        .chars()
        .map(|c| {
            c.to_string()
                .as_str()
                .parse()
                .expect("Could not parse instruction")
        })
        .collect();

    assert!(
        lines.next().is_some_and(|x| x.is_empty()),
        "Did not find expected blank line in input"
    );

    let nodes: HashMap<String, NodeElements> = lines
        .take_while(|line: &&str| !line.is_empty())
        .map(|line| {
            let mut chars = line.chars();
            let chars_ref = chars.by_ref();
            let node = chars_ref
                .take_while(|c| c.is_alphanumeric())
                .collect::<String>();
            let left = chars_ref
                .skip_while(|c| !c.is_alphanumeric())
                .take_while(|c| c.is_alphanumeric())
                .collect::<String>();
            let right = chars_ref
                .skip_while(|c| !c.is_alphanumeric())
                .take_while(|c| c.is_alphanumeric())
                .collect::<String>();
            (node, NodeElements(left, right))
        })
        .collect();

    (instructions, nodes)
}

fn part_one(filename: &str) -> usize {
    let input = read_to_string(filename).expect("Could not read input file");
    let (instructions, nodes) = parse_input(input.as_str());

    let mut current_key = "AAA";
    let mut step_count = 0;
    for instruction in instructions.iter().cycle() {
        let elements = nodes.get(current_key).expect("Could not find expected key");
        current_key = match instruction {
            Instruction::Left => elements.0.as_str(),
            Instruction::Right => elements.1.as_str(),
        };
        step_count += 1;
        if current_key == "ZZZ" {
            break;
        }
    }
    step_count
}

fn part_two(filename: &str) -> usize {
    let input = read_to_string(filename).expect("Could not read input file");
    let (instructions, nodes) = parse_input(input.as_str());
    let mut current_keys: Vec<_> = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| key.as_str())
        .collect();

    let mut step_count = 0;
    for instruction in instructions.iter().cycle() {
        // println!("{}", current_keys.clone().join(","));
        current_keys = current_keys
            .iter()
            .map(|key| {
                let elements = nodes.get(*key).expect("Could not find expected key");
                match instruction {
                    Instruction::Left => elements.0.as_str(),
                    Instruction::Right => elements.1.as_str(),
                }
             })
            .collect();
        step_count += 1;
        if current_keys.iter().all(|key| key.ends_with('Z')) {
            break;
        }
    }

    step_count
}
