extern crate regex;
use regex::Regex;
use std::io::prelude::*;
use std::fs::File;

fn input() -> Option<String> {
    match File::open("input.txt") {
        Ok(mut f) => {
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Some(s),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    length: i32,
}

impl Step {
    fn new(raw_input: &str, re: &Regex) -> Step {
        let cap = re.captures(raw_input.trim()).expect("Invalid direction");
        let direction = match cap.at(1) {
            Some("L") => Direction::Left,
            Some("R") => Direction::Right,
            Some(_) => panic!("wtf!?"),
            None => panic!("wtf!?"),
        };

        let num = match cap.at(2) {
            Some(num) => num.parse::<i32>().expect("Invalid input"),
            None => panic!("wtf!?"),
        };

        Step {
            direction: direction,
            length: num,
        }
    }
}

#[derive(Debug, PartialEq)]
enum CompassDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Position {
    facing: CompassDirection,
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        // don't want to consider other attributes, like "facing"
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    fn follow(&self, step: Step) -> Position {
        let new_direction = match step.direction {
            Direction::Left => {
                match self.facing {
                    CompassDirection::North => CompassDirection::West,
                    CompassDirection::East => CompassDirection::North,
                    CompassDirection::South => CompassDirection::East,
                    CompassDirection::West => CompassDirection::South,
                }
            }
            Direction::Right => {
                match self.facing {
                    CompassDirection::North => CompassDirection::East,
                    CompassDirection::East => CompassDirection::South,
                    CompassDirection::South => CompassDirection::West,
                    CompassDirection::West => CompassDirection::North,
                }
            }
        };

        match new_direction {
            CompassDirection::North => {
                Position {
                    facing: new_direction,
                    x: self.x,
                    y: self.y + step.length,
                }
            }
            CompassDirection::East => {
                Position {
                    facing: new_direction,
                    x: self.x + step.length,
                    y: self.y,
                }
            }
            CompassDirection::South => {
                Position {
                    facing: new_direction,
                    x: self.x,
                    y: self.y - step.length,
                }
            }
            CompassDirection::West => {
                Position {
                    facing: new_direction,
                    x: self.x - step.length,
                    y: self.y,
                }
            }
        }
    }
}

#[derive(Debug)]
struct Trail {
    path: Vec<Position>,
}

impl Trail {
    fn new() -> Trail {
        let start = Position {
            facing: CompassDirection::North,
            x: 0,
            y: 0,
        };
        Trail { path: vec![start] }
    }

    fn follow_instructions(&mut self, instructions: String) {
        let re = Regex::new(r"^([LR])(\d*)$").expect("Could not compile regex");
        let steps = instructions.split(", ").map(|step| Step::new(step, &re));
        for step in steps {
            self.add_by_following(step);
        }
    }

    fn add_by_following(&mut self, step: Step) {
        let new_position = self.current_position().follow(step);
        if self.path.iter().any(|pos| pos == &new_position) {
            println!("Wow, we've been here before! {:?}", new_position);
            println!("Distance: {}", self.distance(&new_position));
        }
        self.path.push(new_position);
    }

    fn current_position(&self) -> &Position {
        self.path.last().expect("must not be empty")
    }

    fn summarize_position(&self) {
        let pos = self.current_position();
        println!("Current position is {:?}", pos);
        println!("Which is {} blocks from the starting position", self.distance(pos));
    }

    fn distance(&self, position: &Position) -> i32 {
        position.x.checked_abs().expect("uh") + position.y.checked_abs().expect("oo")
    }
}

fn main() {
    let instructions = input().expect("uh, couldn't read file");

    let mut trail = Trail::new();
    trail.follow_instructions(instructions);
    trail.summarize_position();
}
