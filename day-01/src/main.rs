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

#[derive(Copy, Clone, Debug)]
enum TurningDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct Step {
    direction: TurningDirection,
    length: i32,
}

impl Step {
    fn new(raw_input: &str, re: &Regex) -> Step {
        let cap = re.captures(raw_input.trim()).expect("Invalid direction");
        let direction = match cap.at(1) {
            Some("L") => TurningDirection::Left,
            Some("R") => TurningDirection::Right,
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

#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn follow(&self, step: Step) -> Vec<Position> {
        let new_direction = match step.direction {
            TurningDirection::Left => {
                match self.facing {
                    CompassDirection::North => CompassDirection::West,
                    CompassDirection::East => CompassDirection::North,
                    CompassDirection::South => CompassDirection::East,
                    CompassDirection::West => CompassDirection::South,
                }
            }
            TurningDirection::Right => {
                match self.facing {
                    CompassDirection::North => CompassDirection::East,
                    CompassDirection::East => CompassDirection::South,
                    CompassDirection::South => CompassDirection::West,
                    CompassDirection::West => CompassDirection::North,
                }
            }
        };

        (1..step.length + 1)
            .map(|z| {
                // FIXME: renmame this dumb var name...
                match new_direction {
                    CompassDirection::North => {
                        Position {
                            facing: new_direction,
                            x: self.x,
                            y: self.y + z,
                        }
                    }
                    CompassDirection::East => {
                        Position {
                            facing: new_direction,
                            x: self.x + z,
                            y: self.y,
                        }
                    }
                    CompassDirection::South => {
                        Position {
                            facing: new_direction,
                            x: self.x,
                            y: self.y - z,
                        }
                    }
                    CompassDirection::West => {
                        Position {
                            facing: new_direction,
                            x: self.x - z,
                            y: self.y,
                        }
                    }
                }
            })
            .collect()
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
        for new_position in self.current_position().follow(step) {
            self.path.push(new_position);
        }
    }

    fn first_visited_twice_distance(&self) -> Option<i32> {
        for (pos, i) in self.path.iter().zip(0..) {
            let found_duplicate = self.path.iter().skip(i + 1).find(|later_pos| &pos == later_pos);
            match found_duplicate {
                Some(_) => return Some(self.distance(pos)),
                None => {}
            }
        }
        None
    }

    fn current_position(&self) -> &Position {
        self.path.last().expect("must not be empty")
    }

    fn summarize_position(&self) {
        let pos = self.current_position();
        println!("Current position is {:?}", pos);
        println!("Which is {} blocks from the starting position",
                 self.distance(pos));
    }

    fn summarize_first_visited_twice_position(&self) {
        println!("First location I visited twice is {} blocks from the starting position",
                 self.first_visited_twice_distance().unwrap());
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
    trail.summarize_first_visited_twice_position();
}

#[test]
fn test_first_visited_twice_distancee() {
    let mut trail = Trail::new();
    trail.follow_instructions(String::from("R8, R4, R4, R8"));
    assert_eq!(trail.first_visited_twice_distance(), Some(4));
}

#[test]
fn test_sad_first_visited_twice_distancee() {
    let mut trail = Trail::new();
    trail.follow_instructions(String::from("R8, R4"));
    assert_eq!(trail.first_visited_twice_distance(), None);
}
