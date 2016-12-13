mod message;
use message::Message;

fn main() {
    let message = Message::new("input.txt").message();
    println!("{}", message);
}

#[test]
fn example() {
    assert_eq!(Message::new("example.txt").message(), "advent");
}
