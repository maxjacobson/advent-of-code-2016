use std::io::prelude::*;
use std::fs::File;
use keypad::Keypad;

pub struct BathroomCode {
    pub val: String,
}

impl BathroomCode {
    pub fn new(filename: String) -> BathroomCode {
        let mut f = File::open(filename).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let mut keypad = Keypad::new((1, 1));

        let val: String = s.lines()
            .map(|line| {
                for directive in line.chars() {
                    keypad = keypad.adjust(directive);
                }

                keypad.current_button().unwrap()
            })
            .collect();

        BathroomCode { val: val }
    }
}
