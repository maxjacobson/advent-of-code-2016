extern crate regex;
use self::regex::Regex;

#[derive(Debug)]
pub struct IpAddress {
    pub raw_input: String,
    raw_input_pattern: Regex,
}

impl IpAddress {
    pub fn new(raw_input: String) -> IpAddress {
        let raw_input_pattern = Regex::new(r"(\w+)?\[(\w+)\](\w+)?").unwrap();
        IpAddress {
            raw_input: raw_input,
            raw_input_pattern: raw_input_pattern,
        }
    }

    pub fn tls(&self) -> bool {
        let mut abbas = vec![];
        let mut brackets_abbas = vec![];
        for cap in self.raw_input_pattern.captures_iter(&self.raw_input) {
            let pre_brackets_text = cap.at(1).unwrap_or("");
            abbas.push(pre_brackets_text);
            let in_brackets_text = cap.at(2).unwrap();
            brackets_abbas.push(in_brackets_text);
            let post_brackets_text = cap.at(3).unwrap_or("");
            abbas.push(post_brackets_text);
        }
        let any_outside_of_brackets = abbas.iter().any(|a| self.is_abba(a));
        let any_in_brackets = brackets_abbas.iter().any(|a| self.is_abba(a));

        any_outside_of_brackets && !any_in_brackets
    }

    fn is_abba(&self, abba: &str) -> bool {
        for i in 0..abba.chars().count() {
            let a1 = abba.chars().nth(i);
            let b1 = abba.chars().nth(i + 1);
            let b2 = abba.chars().nth(i + 2);
            let a2 = abba.chars().nth(i + 3);
            if a1.is_none() {
                continue;
            }
            if b1.is_none() {
                continue;
            }
            if b2.is_none() {
                continue;
            }
            if a2.is_none() {
                continue;
            }

            if a1.unwrap() == a2.unwrap() && b1.unwrap() == b2.unwrap() &&
               a1.unwrap() != b1.unwrap() {
                return true;
            }
        }
        false
    }
}

#[test]
fn normal_tls() {
    // abba[mnop]qrst supports TLS (abba outside square brackets).
    let ip = IpAddress::new(String::from("abba[mnop]qrst"));
    assert!(ip.tls());
}

#[test]
fn brackets_overriding_outside() {
    // abcd[bddb]xyyx does not support TLS (bddb is within square
    // brackets, even though xyyx is outside square brackets).
    let ip = IpAddress::new(String::from("abcd[bddb]xyyx"));
    assert!(!ip.tls());
}

#[test]
fn not_all_anagrams() {
    // aaaa[qwer]tyui does not support TLS (aaaa is invalid;
    // the interior characters must be different).
    let ip = IpAddress::new(String::from("aaaa[qwer]tyui"));
    assert!(!ip.tls());
}

#[test]
fn hidden_within_strings() {
    // ioxxoj[asdfgh]zxcvbn supports TLS (oxxo is outside square brackets
    // , even though it's within a larger string).
    let ip = IpAddress::new(String::from("ioxxoj[asdfgh]zxcvbn"));
    assert!(ip.tls());
}
