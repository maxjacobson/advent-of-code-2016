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
        },
        Err(_) => None,
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Length {
    num: i32,
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    length: Length,
}

impl Step {
    fn new(raw_input: &str, re: &Regex) -> Step {
        let cap = re.captures(raw_input).expect("Invalid direction");
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
            length: Length { num: num },
        }
    }
}

#[derive(Debug)]
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

impl Position {
    fn follow(&mut self, step: Step) {
        match step.direction {
            Direction::Left => {
                match self.facing {
                    CompassDirection::North => {
                        self.facing = CompassDirection::West;
                    },
                    CompassDirection::East => {
                        self.facing = CompassDirection::North;
                    },
                    CompassDirection::South => {
                        self.facing = CompassDirection::East;
                    },
                    CompassDirection::West => {
                        self.facing = CompassDirection::South;
                    },
                }
            },
            Direction::Right => {
                match self.facing {
                    CompassDirection::North => {
                        self.facing = CompassDirection::East;
                    },
                    CompassDirection::East => {
                        self.facing = CompassDirection::South;
                    },
                    CompassDirection::South => {
                        self.facing = CompassDirection::West;
                    },
                    CompassDirection::West => {
                        self.facing = CompassDirection::North;
                    },
                }
            },
        }

        match self.facing {
            CompassDirection::North => {
                self.y = self.y + step.length.num;
            },
            CompassDirection::East => {
                self.x = self.x + step.length.num;
            },
            CompassDirection::South => {
                self.y = self.y - step.length.num;
            },
            CompassDirection::West => {
                self.x = self.x - step.length.num;
            },
        }
    }
}

fn distance(position: Position) -> i32 {
    position.x.checked_abs().expect("uh") + position.y.checked_abs().expect("oo")
}

fn main() {
    let instructions = input().expect("uh, couldn't read file");
    let re = Regex::new(r"^([LR])(\d*)$").expect("Could not compile regex");

    let mut position = Position { facing: CompassDirection::North, x: 0, y: 0 };

    let steps = instructions.split(", ").map(|step| Step::new(step.trim_right(), &re));
    for step in steps {
        position.follow(step);
        println!("{:?}", position);
    }

    println!("Final destination is {} blocks from starting position", distance(position));
}
