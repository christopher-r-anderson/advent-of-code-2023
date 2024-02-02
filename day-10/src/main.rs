use std::{fs::read_to_string, str::FromStr};

fn main() {
    println!("Part One Ex (8): {}", part_one("example.txt"));
    println!("Part One Input: {}", part_one("input.txt"));

    // println!("Part Two Ex (2): {}", part_two("example.txt"));
    // println!("Part Two Input: {}", part_two("input.txt"));
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }
    fn get_opposite(dir: &Direction) -> Direction {
        match dir {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Terrain {
    Start,
    Ground,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Terrain {
    fn connects(&self, dir: &Direction) -> bool {
        match dir {
            Direction::North => matches!(self, Terrain::Start | Terrain::Vertical | Terrain::NorthEast | Terrain::NorthWest),
            Direction::South => matches!(self, Terrain::Start | Terrain::Vertical | Terrain::SouthEast | Terrain::SouthWest),
            Direction::West => matches!(self, Terrain::Start | Terrain::Horizontal | Terrain::NorthWest | Terrain::SouthWest),
            Direction::East => matches!(self, Terrain::Start | Terrain::Horizontal | Terrain::NorthEast | Terrain::SouthEast),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseTerrainError(String);

impl FromStr for Terrain {
    type Err = ParseTerrainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Terrain::Start),
            "." => Ok(Terrain::Ground),
            "|" => Ok(Terrain::Vertical),
            "-" => Ok(Terrain::Horizontal),
            "L" => Ok(Terrain::NorthEast),
            "J" => Ok(Terrain::NorthWest),
            "F" => Ok(Terrain::SouthEast),
            "7" => Ok(Terrain::SouthWest),
            _ => Err(ParseTerrainError(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Location(i32, i32);

#[derive(Debug, Eq, PartialEq)]
struct Map(Vec<Vec<Terrain>>);

impl Map {
    fn new(input: String) -> Result<Self, ParseMapError> {
        input.parse()
    }
    fn get_at(&self, loc: &Location) -> &Terrain {
        &self.0[loc.1 as usize][loc.0 as usize]
    }
    fn get_location(&self, x: i32, y: i32) -> Option<Location> {
        if x < 0 || y < 0 || y > self.0.len() as i32 - 1 || x > self.0[y as usize].len() as i32 - 1 {
            None
        } else {
            Some(Location(x, y))
        }
    }
    fn get_start(&self) -> Result<Location, String> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == &Terrain::Start {
                    return Ok(Location(x as i32, y as i32));
                }
            }
        }
        Err("Could not find Start".to_string())
    }
    fn get_neighbor(&self, loc: &Location, dir: &Direction) -> Option<Location> {
        match dir {
            Direction::North => self.get_location(loc.0, loc.1 - 1),
            Direction::South => self.get_location(loc.0, loc.1 + 1),
            Direction::West => self.get_location(loc.0 - 1, loc.1),
            Direction::East => self.get_location(loc.0 + 1, loc.1),
        }
    }
    fn calculate_pipe_length(&self) -> i32 {
        let mut length = 0;
        let mut current = self.get_start().unwrap();
        let mut last_move: Option<Direction> = None;
        'outer: loop {
            length += 1;
            for dir in Direction::get_all() {
                if last_move.as_ref().is_some_and(|last_move| Direction::get_opposite(last_move) == dir) {
                    continue;
                }
                if self.get_at(&current).connects(&dir) && self.get_neighbor(&current, &dir).is_some_and(|next| self.get_at(&next).connects(&Direction::get_opposite(&dir))) {
                    // println!("going {:?} from '{:?}' to '{:?}'", &dir, self.get_at(&current), self.get_at(&self.get_neighbor(&current, &dir).unwrap()));
                    current = self.get_neighbor(&current, &dir).unwrap();
                    last_move = Some(dir);
                    if self.get_at(&current) == &Terrain::Start {
                        break 'outer;
                    } else {
                        continue 'outer;
                    }
                }
            }
        }
        length
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMapError;

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s
                .lines()
                .map(|line| {
                    line.chars().map(|c| c.to_string().parse().unwrap()).collect()
                })
                .collect()
        ))
    }
}

fn part_one(filename: &str) -> i32 {
    let input = read_to_string(filename).expect("Could not read input file");
    let map = Map::new(input).expect("Could not parse Map");
    // round up with int division
    (map.calculate_pipe_length() + 1) / 2
}
