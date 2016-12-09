#[derive(Debug)]
pub struct Button {
    pub val: char,
}

impl Button {
    pub fn new(c: char) -> Button {
        Button { val: c }
    }
}
