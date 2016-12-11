mod kiosk;
use kiosk::Kiosk;
mod room;

fn main() {
    let kiosk = Kiosk::new(String::from("input.txt"));
    println!("Sum of real room sector ids: {}", kiosk.sector_id_sums());

    for room in kiosk.rooms {
        println!("{:?} decrypted: {}", room, room.decrypt());
    }
}
