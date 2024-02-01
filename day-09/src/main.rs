use std::fs::read_to_string;

fn main() {
    println!("Part One Ex (114): {}", part_one("example.txt"));
    println!("Part One Input: {}", part_one("input.txt"));

    println!("Part Two Ex (2): {}", part_two("example.txt"));
    println!("Part Two Input: {}", part_two("input.txt"));
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Backward,
    Forward,
}

fn get_next_entry(filename: &str, dir: Direction) -> i32 {
    let input = read_to_string(filename).expect("Could not read input file");
    input
        .lines()
        .map(|line| {
            let mut start_seq: Vec<i32> = line
                .split_whitespace()
                .map(|item| item.parse().expect("Could not parse input number"))
                .collect();
            if dir == Direction::Backward {
                start_seq.reverse();
            }
            let mut sequences = vec![start_seq];
            loop {
                let previous_seq = sequences.last().unwrap();
                let mut index_in_seq = 0;
                // -1 because each round is shorter +1 because we are filling one back in
                let mut next_seq = Vec::with_capacity(previous_seq.len());
                while index_in_seq < previous_seq.len() - 1 {
                    let a = previous_seq[index_in_seq];
                    index_in_seq += 1;
                    let b = previous_seq[index_in_seq];
                    if dir == Direction::Backward {
                        next_seq.push(a - b);
                    } else {
                        next_seq.push(b - a);
                    }
                    
                }
                let is_zeroed = next_seq.iter().all(|entry| *entry == 0);
                sequences.push(next_seq);
                if is_zeroed {
                    break;
                }
            }

            // not pushing extra zero on the end since the last row is all zeros
            let mut seq_index = sequences.len() - 1;
            loop {
                let a = sequences[seq_index].last().unwrap();
                seq_index -= 1;
                let b = sequences[seq_index].last().unwrap();
                let new_value = if dir == Direction::Backward {
                    b - a
                } else {
                    b + a
                };
                if seq_index == 0 {
                    break new_value;
                } else {
                    sequences[seq_index].push(new_value);
                }
            }
        })
        .sum()
}

fn part_one(filename: &str) -> i32 {
    get_next_entry(filename, Direction::Forward)
}

fn part_two(filename: &str) -> i32 {
    get_next_entry(filename, Direction::Backward)
}
