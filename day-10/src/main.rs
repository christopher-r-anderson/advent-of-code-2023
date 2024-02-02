use std::{fs::read_to_string, str::FromStr};

fn main() {
    println!("Part One Ex (8): {}", part_one("example-1.txt"));
    println!("Part One Input: {}", part_one("input.txt"));

    println!("Part Two Ex A (4): {}", part_two("example-2a.txt"));
    println!("Part Two Ex B (8): {}", part_two("example-2b.txt"));
    println!("Part Two Ex C (10): {}", part_two("example-2c.txt"));
    println!("Part Two Input: {}", part_two("input.txt"));
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
    fn dimensions(&self) -> (usize, usize) {
        let height = self.0.len();
        if height > 0 {
            (self.0[0].len(), height)
        } else {
            (0, 0)
        }
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
    fn update_start(&mut self, terrain: Terrain) {
        let start = self.get_start().unwrap();
        let row = &mut self.0[start.1 as usize];
        row[start.0 as usize] = terrain;
    }
    fn get_neighbor(&self, loc: &Location, dir: &Direction) -> Option<Location> {
        match dir {
            Direction::North => self.get_location(loc.0, loc.1 - 1),
            Direction::South => self.get_location(loc.0, loc.1 + 1),
            Direction::West => self.get_location(loc.0 - 1, loc.1),
            Direction::East => self.get_location(loc.0 + 1, loc.1),
        }
    }
    fn calculate_pipe(&mut self) -> Vec<Location> {
        let mut last_move: Option<Direction> = None;
        let mut path = vec![self.get_start().unwrap()];
        let mut start_exit = None;
        'outer: loop {
            for dir in Direction::get_all() {
                if last_move.as_ref().is_some_and(|last_move| Direction::get_opposite(last_move) == dir) {
                    continue;
                }
                let current = path.last().unwrap();
                if self.get_at(current).connects(&dir) && self.get_neighbor(current, &dir).is_some_and(|next| self.get_at(&next).connects(&Direction::get_opposite(&dir))) {
                    if start_exit.is_none() {
                        start_exit = Some(dir.clone());
                    }
                    let next = self.get_neighbor(current, &dir).unwrap();
                    last_move = Some(dir.clone());
                    if self.get_at(&next) == &Terrain::Start {
                        let start = match (start_exit.unwrap(), Direction::get_opposite(&dir)) {
                            (Direction::North, Direction::South) | (Direction::South, Direction::North) => Terrain::Vertical,
                            (Direction::East, Direction::West) | (Direction::West, Direction::East) => Terrain::Horizontal,
                            (Direction::North, Direction::East) | (Direction::East, Direction::North) => Terrain::NorthEast,
                            (Direction::North, Direction::West) | (Direction::West, Direction::North) => Terrain::NorthWest,
                            (Direction::South, Direction::East) | (Direction::East, Direction::South) => Terrain::SouthEast,
                            (Direction::South, Direction::West) | (Direction::West, Direction::South) => Terrain::SouthWest,
                            _ => panic!("Not a possible starting terrain"),
                        };
                        self.update_start(start);
                        break 'outer;
                    } else {
                        path.push(next);
                        continue 'outer;
                    }
                }
            }
        }
        path
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
    let mut map = Map::new(input).expect("Could not parse Map");
    // round up with int division
    (map.calculate_pipe().len() as i32 + 1) / 2
}

// TODO: Really slow on full input - see if optimizations are possible
fn part_two(filename: &str) -> i32 {
    let input = read_to_string(filename).expect("Could not read input file");
    let mut map = Map::new(input).expect("Could not parse Map");
    let path = map.calculate_pipe();
    let mut inside_count = 0;
    let (width, height) = map.dimensions();
    for x in 0..width {
        for y in 0..height {
            if path.contains(&Location(x as i32, y as i32)) {
                continue;
            } else {
                let mut crossings = 0;
                let mut line_entry = None;
                for x in 0..(x as i32) {
                    let loc = &Location(x, y as i32);
                    if !path.contains(loc) {
                        continue;
                    }
                    let terrain = map.get_at(loc);
                    match *terrain {
                        Terrain::Horizontal | Terrain::Ground => {},
                        Terrain::Vertical => {
                            crossings += 1
                        },
                        Terrain::NorthEast => line_entry = Some(Direction::North),
                        Terrain::SouthEast => line_entry = Some(Direction::South),
                        Terrain::NorthWest => {
                            if line_entry == Some(Direction::South) {
                                crossings += 1;
                                line_entry = None;
                            }
                        },
                        Terrain::SouthWest => {
                            if line_entry == Some(Direction::North) {
                                crossings += 1;
                                line_entry = None;
                            }
                        },
                        Terrain::Start => panic!("Start should have been replaced"),
                    }
                }
                if crossings % 2 == 1 {
                    inside_count += 1;
                }
            }
        }
    }
    inside_count
}
