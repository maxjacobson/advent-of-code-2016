extern crate regex;
use regex::Regex;
use std::io::prelude::*;
use std::fs::File;

struct Decompressing;

impl Decompressing {
    fn run(input: String) -> String {
        let mut result = String::new();
        let pattern = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

        let mut captures = pattern.captures_iter(&input);
        if let Some(first) = captures.nth(0) {
            let pos = first.pos(0).unwrap();
            result.push_str(&input[0..pos.0]);

            let characters_to_consume: usize= first.at(1).unwrap().parse().unwrap();

            let chars = &input[pos.1.. pos.1 + characters_to_consume];

            let times_to_repeat: i32 = first.at(2).unwrap().parse().unwrap();

            for _ in 0..times_to_repeat {
                result.push_str(&chars);
            }

            let remaining_input = &input[pos.1 + characters_to_consume ..];

            return result + &Decompressing::run(remaining_input.to_owned());
        } else {
            result.push_str(&input);
            return result;
        }
    }
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let decompressed = Decompressing::run(s.trim().to_owned());
    println!("Decompressed: {}", decompressed);
    println!("Length: {}", decompressed.len());
}

#[test]
fn advent() {
    assert_eq!(Decompressing::run(format!("ADVENT")), "ADVENT");
}

#[test]
fn one_marker() {
    assert_eq!(Decompressing::run(format!("A(1x5)BC")), "ABBBBBC");
}

#[test]
fn marker_at_beginning() {
    assert_eq!(Decompressing::run(format!("(3x3)XYZ")), "XYZXYZXYZ");
}

#[test]
fn multiple_markers() {
    assert_eq!(Decompressing::run(format!("A(2x2)BCD(2x2)EFG")), "ABCBCDEFEFG");
}

#[test]
fn trick_marker() {
    assert_eq!(Decompressing::run(format!("(6x1)(1x3)A")), "(1x3)A");
}

#[test]
fn multiple_trick_markers() {
    assert_eq!(Decompressing::run(format!("X(8x2)(3x3)ABCY")), "X(3x3)ABC(3x3)ABCY");
}
