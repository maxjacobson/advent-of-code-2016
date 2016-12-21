use std::io::prelude::*;
use std::fs::File;
use ip_address::IpAddress;

pub struct AddressList {
    addresses: Vec<IpAddress>,
}

impl AddressList {
    pub fn new(filename: String) -> AddressList {
        let mut addresses = vec![];
        let mut f = File::open(filename).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        for line in s.lines() {
            addresses.push(IpAddress::new(String::from(line)));
        }

        AddressList { addresses: addresses }
    }

    pub fn tls_count(&self) -> usize {
        self.addresses.iter().filter(|address| address.tls()).count()
    }
}

#[test]
fn example() {
    let list = AddressList::new(String::from("example.txt"));
    assert_eq!(list.tls_count(), 2);
}
