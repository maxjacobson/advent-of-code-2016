extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

pub struct Password {
    door_id: String,
}

impl Password {
    pub fn new(door_id: String) -> Password {
        Password { door_id: door_id }
    }

    pub fn val(&self) -> String {
        (0..)
            .filter_map(|n| {
                let hash = self.hashed_counter(n);
                match hash.starts_with("00000") {
                    true => Some(hash.chars().nth(5).unwrap()),
                    false => None,
                }
            })
            .take(8)
            .collect()
    }

    fn hashed_counter(&self, counter: u32) -> String {
        let input = format!("{}{}", self.door_id, counter);
        self.hashed(input)
    }

    fn hashed(&self, input: String) -> String {
        let mut md5 = Md5::new();
        md5.input_str(&input);
        md5.result_str()
    }
}

#[test]
fn hashed_counter() {
    let password = Password::new(String::from("abc"));
    let hash = password.hashed_counter(3231929);
    assert!(hash.starts_with("00000"));
}
