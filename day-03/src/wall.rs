use triangle::Triangle;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct Wall {
    triangles: Vec<Triangle>,
}

impl Wall {
    pub fn new(filename: String) -> Wall {
        let mut f = File::open(filename).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let mut triangles: Vec<Triangle> = vec![];

        for current_col in 0..3 {
            let mut column_values: Vec<String> = vec![];
            for line in s.lines() {
                let mut row_components = line.split_whitespace();
                let row_value = row_components.nth(current_col).unwrap();
                column_values.push(row_value.to_owned());
            }

            for chunk in column_values.chunks(3) {
                let triangle = Triangle::new(chunk[0].to_owned(),
                                             chunk[1].to_owned(),
                                             chunk[2].to_owned());
                triangles.push(triangle);
            }
        }

        Wall { triangles: triangles }
    }

    pub fn possible_triangles_count(&self) -> usize {
        self.triangles.iter().filter(|t| t.possible()).count()
    }
}

#[test]
fn loading_file_by_column() {
    let wall = Wall::new(String::from("example.txt"));
    assert_eq!(wall.triangles.get(0).unwrap(),
               &Triangle {
                   a: 101,
                   b: 102,
                   c: 103,
               });
    assert_eq!(wall.triangles.get(5).unwrap(),
               &Triangle {
                   a: 601,
                   b: 602,
                   c: 603,
               });
}
