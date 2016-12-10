mod triangle;
mod wall;
use wall::Wall;

fn main() {
    let wall = Wall::new(String::from("input.txt"));
    println!("Number possible triangles on wall: {}",
             wall.possible_triangles_count());
}

#[test]
fn example() {
    let wall = Wall::new(String::from("example.txt"));
    assert_eq!(wall.possible_triangles_count(), 1);
}
