use triangle::Triangle;
use std::io::prelude::*;
use std::fs::File;

pub struct Wall {
    triangles: Vec<Triangle>,
}

impl Wall {
    pub fn new(filename: String) -> Wall {
        let mut f = File::open(filename).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let mut triangles: Vec<Triangle> = vec![];
        for line in s.lines() {
            let mut components = line.split_whitespace();
            let triangle = Triangle::new(components.next().unwrap().to_string(),
                                         components.next().unwrap().to_string(),
                                         components.next().unwrap().to_string());
            triangles.push(triangle);
        }

        Wall { triangles: triangles }
    }

    pub fn possible_triangles_count(&self) -> usize {
        self.triangles.iter().filter(|t| t.possible()).count()
    }
}
