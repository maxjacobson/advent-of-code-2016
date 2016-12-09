mod bathroom_code;
mod button;
mod keypad;
use bathroom_code::BathroomCode;

fn main() {
    let bathroom_code = BathroomCode::new(String::from("input.txt"));
    println!("The answer is: {}", bathroom_code.val);
}

#[test]
fn example() {
    let bathroom_code = BathroomCode::new(String::from("example.txt"));
    assert_eq!(bathroom_code.val, String::from("1985"));
}
