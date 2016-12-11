use std::io::prelude::*;
use std::fs::File;

use room::Room;

#[derive(Debug)]
pub struct Kiosk {
    rooms: Vec<Room>,
}

impl Kiosk {
    pub fn new(filename: String) -> Kiosk {
        let mut f = File::open(filename).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let mut rooms: Vec<Room> = vec![];
        for line in s.lines() {
            let room = Room::new(line.to_owned());
            rooms.push(room);
        }
        Kiosk { rooms: rooms }
    }

    pub fn sector_id_sums(&self) -> u32 {
        self.rooms.iter().filter(|room| room.real()).fold(0, |sum, ref room| sum + room.sector_id)
    }
}

#[test]
fn example() {
    let kiosk = Kiosk::new(String::from("example.txt"));
    assert_eq!(kiosk.sector_id_sums(), 1514);
}
