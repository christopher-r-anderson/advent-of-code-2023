use std::fs::read_to_string;

fn main() {
    println!("Part One Ex: {}", part_one(get_input("example-1.txt")));
    println!("Part One Real: {}", part_one(get_input("input.txt")));

    println!("Part Two Ex: {}", part_two(get_input("example-2.txt")));
    println!("Part Two Real: {}", part_two(get_input("input.txt")));
}

fn get_input(filename: &str) -> String {
    read_to_string(filename)
        .expect("Could not read input file")
}

fn part_one(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c: &char| c.is_numeric()).unwrap();
            let second = line.chars().rfind(|c: &char| c.is_numeric()).unwrap();
            format!("{first}{second}").parse::<i64>().unwrap()
        })
        .sum::<i64>()
}

fn get_value(string: &str) -> Option<u64> {
    match string {
        _ if string.ends_with("one") => Some(1),
        _ if string.ends_with("two") => Some(2),
        _ if string.ends_with("three") => Some(3),
        _ if string.ends_with("four") => Some(4),
        _ if string.ends_with("five") => Some(5),
        _ if string.ends_with("six") => Some(6),
        _ if string.ends_with("seven") => Some(7),
        _ if string.ends_with("eight") => Some(8),
        _ if string.ends_with("nine") => Some(9),
        _ => {
            let last_char = string.chars().last().unwrap();
            if last_char.is_ascii_digit() {
                Some(last_char.to_string().parse::<u64>().unwrap())
            } else {
                None
            }
        },
    }
}

fn get_rvalue(string: &str) -> Option<u64> {
    match string {
        _ if string.starts_with("one") => Some(1),
        _ if string.starts_with("two") => Some(2),
        _ if string.starts_with("three") => Some(3),
        _ if string.starts_with("four") => Some(4),
        _ if string.starts_with("five") => Some(5),
        _ if string.starts_with("six") => Some(6),
        _ if string.starts_with("seven") => Some(7),
        _ if string.starts_with("eight") => Some(8),
        _ if string.starts_with("nine") => Some(9),
        _ => {
            let first_char = string.chars().nth(0).unwrap();
            if first_char.is_ascii_digit() {
                Some(first_char.to_string().parse::<u64>().unwrap())
            } else {
                None
            }
        },
    }
}

fn find_value(string: &str) -> u64 {
    let len = string.len();
    for i in 0..len {
        if let Some(v) = get_value(&string[0 .. i + 1]) {
            return v;
        }
    }
    panic!("Could not find number");
}

fn find_rvalue(string: &str) -> u64 {
    let len = string.len();
    for i in 0..len {
        if let Some(v) = get_rvalue(&string[len - i - 1 ..]) {
            return v;
        }
    }
    panic!("Could not find number");
}

fn part_two(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            let first = find_value(line);
            let second = find_rvalue(line);
            format!("{first}{second}").parse::<i64>().unwrap()
        })
        .sum::<i64>()
}
