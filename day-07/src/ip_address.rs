extern crate regex;
use self::regex::Regex;

struct IpAddress {
    raw_input: String,
    re: Regex,
    other_re: Regex,
}

impl IpAddress {
    fn new(raw_input: String) -> IpAddress {
        // FIXME: make this one pattern?
        let re = Regex::new(r"(\w+)\[(\w+)\](\w+)?").unwrap();
        let other_re = Regex::new(r"(\w)(\w)\2\1").unwrap();
        IpAddress {
            raw_input: raw_input,
            re: re,
            other_re: other_re,
        }
    }

    fn tls(&self) -> bool {
        let mut abbas = vec![];
        let mut brackets_abbas = vec![];
        for cap in self.re.captures_iter(&self.raw_input) {
            let pre_brackets_text = cap.at(1).unwrap();
            abbas.push(pre_brackets_text);
            let in_brackets_text = cap.at(2).unwrap();
            brackets_abbas.push(in_brackets_text);
            let post_brackets_text = cap.at(3).unwrap_or("");
            abbas.push(post_brackets_text);
        }
        // println!("match: {:?}", cap);
        // println!("{:?}", abbas);
        abbas.iter().any(|a| self.is_abba(a)) && !brackets_abbas.iter().any(|a| self.is_abba(a))
    }

    fn is_abba(&self, abba: &str) -> bool {
        // println!("{:?}", abba);
        // self.other_re.is_match(abba)
        for cap in self.other_re.captures_iter(abba) {
            println!("GOT IT {:?}", abba);
            let the_match = cap.at(0).unwrap();
            if the_match.chars().nth(0).unwrap() != the_match.chars().nth(1).unwrap() {
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

#[test]
fn pattern() {
    let ip = IpAddress::new(String::from("ioxxoj[asdfgh]zxcvbn"));
    assert!(ip.other_re.is_match(&ip.raw_input));
}
