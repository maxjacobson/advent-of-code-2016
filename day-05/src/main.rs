mod password;
use password::Password;

fn main() {
    let password = Password::new(String::from("ugkcyxxp"));
    println!("Password: {}", password.val());
}

#[test]
fn example() {
    let password = Password::new(String::from("abc"));
    assert_eq!(password.val(), "18f47a30");
}
