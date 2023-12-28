use std::{fs::read_to_string, str::FromStr, collections::HashMap};

fn main() {
    println!("Part One Ex: {:?}", part_one("example-1.txt"));
    println!("Part One Real: {:?}", part_one("input.txt"));

    // println!("Part Two Ex: {}", part_two("example-2.txt"));
    // println!("Part Two Real: {}", part_two("input.txt"));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            Err("Invalid card count".to_string())?
        }
        let cards = s
            .chars()
            .map(|c| {
                match c {
                    '2'..='9' => Ok(c.to_digit(10).unwrap()),
                    'T' => Ok(10),
                    'J' => Ok(11),
                    'Q' => Ok(12),
                    'K' => Ok(13),
                    'A' => Ok(14),
                    _ => Err("Invalid card value".to_string()),
                }
            })
            .collect::<Result<Vec<u32>, String>>()?;
        let mut counts: HashMap<&u32, i32> = HashMap::new();
        for card in &cards {
            match counts.get_mut(card) {
                Some(card) => *card += 1,
                None => {
                    counts.insert(card, 1);
                },
            };
        }
        let dups = counts.values().collect::<Vec<_>>();
        let hand_type = if dups.contains(&&5) {
            HandType::FiveOfAKind
        } else if dups.contains(&&4) {
            HandType::FourOfAKind
        } else if dups.contains(&&3) {
            if dups.contains(&&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if dups.contains(&&2) {
            if dups.iter().filter(|x| ***x == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        };
        Ok(Hand {
            cards,
            hand_type,
        })
    }
}

fn part_one(filename: &str) -> Result<i32, String> {
    let input = read_to_string(filename)
        .expect("Could not read input file");
    let mut hands = input
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();
            let hand = Hand::from_str(parts[0])?;
            let bid = parts[1].parse::<i32>().map_err(|err| err.to_string())?;
            Ok::<(Hand, i32), String>((hand, bid))
        })
        .collect::<Result<Vec<_>, String>>()?;

    hands.sort();

    let total = hands
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| (index + 1) as i32 * bid)
        .sum();
    Ok(total)
}
