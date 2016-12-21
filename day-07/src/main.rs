mod address_list;
mod ip_address;
use address_list::AddressList;

fn main() {
    let list = AddressList::new(String::from("input.txt"));
    println!("TLS count: {}", list.tls_count());
}
