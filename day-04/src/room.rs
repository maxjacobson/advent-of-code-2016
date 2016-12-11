extern crate regex;
use self::regex::Regex;

use std::collections::HashMap;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Room {
    encrypted_name: String,
    pub sector_id: u32,
    checksum: String,
}

impl Room {
    pub fn new(raw_description: String) -> Room {
        let regex = Regex::new(r"^([a-z-]+)-(\d+)\[([a-z]+)\]$").unwrap();
        let captures = regex.captures(&raw_description).unwrap();
        Room {
            encrypted_name: captures.at(1).unwrap().to_owned(),
            sector_id: captures.at(2).unwrap().parse::<u32>().unwrap(),
            checksum: captures.at(3).unwrap().to_owned(),
        }
    }

    pub fn real(&self) -> bool {
        self.calculated_checksum() == self.checksum
    }

    fn calculated_checksum(&self) -> String {
        let mut character_counts: HashMap<char, u32> = HashMap::new();
        for c in self.encrypted_name.chars() {
            if c != '-' {
                let count = character_counts.entry(c).or_insert(0);
                *count += 1;
            }
        }

        let mut character_counts_vec: Vec<(&char, &u32)> = character_counts.iter().collect();

        character_counts_vec.sort_by(|a, b| {
            if a.1 == b.1 {
                if a.0 == b.0 {
                    Ordering::Equal
                } else if a.0 > b.0 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else if a.1 > b.1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let calculated: String = character_counts_vec.iter().map(|cc| *cc.0).take(5).collect();
        calculated
    }
}

#[test]
fn new() {
    let room = Room::new(String::from("aaaaa-bbb-z-y-x-123[abxyz]"));
    assert_eq!(room.encrypted_name, "aaaaa-bbb-z-y-x");
    assert_eq!(room.sector_id, 123);
    assert_eq!(room.checksum, "abxyz");
}

#[test]
fn real() {
    let room = Room::new(String::from("aaaaa-bbb-z-y-x-123[abxyz]"));
    assert!(room.real());

    let room2 = Room::new(String::from("a-b-c-d-e-f-g-h-987[abcde]"));
    assert!(room2.real());

    let room3 = Room::new(String::from("not-a-real-room-404[oarel]"));
    assert!(room3.real());

    let room4 = Room::new(String::from("totally-real-room-200[decoy]"));
    assert!(!room4.real());
}
