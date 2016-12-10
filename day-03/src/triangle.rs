#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Triangle {
    pub fn new(a: String, b: String, c: String) -> Triangle {
        let sides = [a, b, c];
        let mut numeric_sides: Vec<usize> =
            sides.into_iter().map(|s| s.parse::<usize>().unwrap()).collect();
        numeric_sides.sort();
        Triangle {
            a: numeric_sides.get(0).unwrap().to_owned(),
            b: numeric_sides.get(1).unwrap().to_owned(),
            c: numeric_sides.get(2).unwrap().to_owned(),
        }
    }

    pub fn possible(&self) -> bool {
        self.a + self.b > self.c
    }
}

#[test]
fn sorting_sides_and_coercing_to_numbers() {
    let triangle = Triangle::new(String::from("3"), String::from("5"), String::from("2"));

    assert_eq!(triangle.a, 2);
    assert_eq!(triangle.b, 3);
    assert_eq!(triangle.c, 5);
}

#[test]
fn possible_test() {
    let triangle = Triangle { a: 1, b: 2, c: 3 };

    assert!(!triangle.possible());

    let triangle = Triangle { a: 3, b: 4, c: 5 };
    assert!(triangle.possible());
}
