extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;
use std::collections::HashMap;

pub struct Password {
    door_id: String,
}

impl Password {
    pub fn new(door_id: String) -> Password {
        Password { door_id: door_id }
    }

    pub fn val(&self) -> String {
        let mut result: HashMap<u32, char> = HashMap::new();
        let mut counter = 0;
        loop {
            if result.len() == 8 {
                let mut more_result: Vec<(&u32, &char)> = result.iter().collect();
                more_result.sort_by(|a, b| a.0.cmp(b.0));
                let final_result: String = more_result.iter().map(|r| *r.1).collect();
                println!("{:?}", more_result);
                return final_result;
            }


            let hash = self.hashed_counter(counter);
            if hash.starts_with("00000") {
                match hash.chars().nth(5).unwrap().to_string().parse::<u32>() {
                    Ok(pos) => {
                        if pos < 8 {
                            let val = hash.chars().nth(6).unwrap();
                            result.entry(pos).or_insert(val);
                        }
                    }
                    Err(_) => {}
                }
            }

            counter += 1;
        }
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
