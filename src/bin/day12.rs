use eyre;
use std::str::FromStr;

use Action::*;
use Direction::*;

fn main() -> eyre::Result<()> {
    let actions = aoc_2020::input_lines("12")?
        .iter()
        .map(|s| Action::from_str(s))
        .collect::<Result<Vec<Action>, &str>>()
        .map_err(|e| eyre::eyre!(e))?;

    let ship = actions
        .into_iter()
        .fold(Ship::empty(), |acc, action| acc.steer(action));

    println!("part one\n{}", ship.manhattan_distance());

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_right(self, degrees: i32) -> Direction {
        let n = (degrees / 90) % 4;
        match (self, n) {
            (_, 0) => self,
            (North, 1) | (South, 3) | (West, 2) => East,
            (North, 2) | (East, 1) | (West, 3) => South,
            (North, 3) | (East, 2) | (South, 1) => West,
            (East, 3) | (South, 2) | (West, 1) => North,
            _ => self,
        }
    }

    fn rotate_left(self, degrees: i32) -> Direction {
        let n = (degrees / 90) % 4;
        match (self, n) {
            (_, 0) => self,
            (East, 1) | (South, 2) | (West, 3) => North,
            (South, 1) | (West, 2) | (North, 3) => East,
            (West, 1) | (North, 2) | (East, 3) => South,
            (North, 1) | (East, 2) | (South, 3) => West,
            _ => self,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    N(i32),
    E(i32),
    S(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("str is not long enough");
        }

        let alph = s.chars().nth(0).unwrap();
        let num = s
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .map_err(|_| "error parsing number")?;

        Ok(match alph {
            'N' => N(num),
            'E' => E(num),
            'S' => S(num),
            'W' => W(num),
            'L' => L(num),
            'R' => R(num),
            'F' => F(num),
            _ => return Err("error parsing action"),
        })
    }
}

struct Ship {
    east: i32,
    north: i32,
    direction: Direction,
}

impl Ship {
    fn empty() -> Ship {
        Ship::new(0, 0, East)
    }

    fn new(x: i32, y: i32, direction: Direction) -> Ship {
        Ship {
            east: x,
            north: y,
            direction,
        }
    }

    fn steer(self, action: Action) -> Ship {
        match action {
            N(n) => Ship::new(self.east, self.north + n, self.direction),
            E(n) => Ship::new(self.east + n, self.north, self.direction),
            S(n) => Ship::new(self.east, self.north - n, self.direction),
            W(n) => Ship::new(self.east - n, self.north, self.direction),
            L(n) => Ship::new(self.east, self.north, self.direction.rotate_left(n)),
            R(n) => Ship::new(self.east, self.north, self.direction.rotate_right(n)),
            F(n) => {
                let a = match self.direction {
                    North => N(n),
                    East => E(n),
                    South => S(n),
                    West => W(n),
                };

                self.steer(a)
            }
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }
}
