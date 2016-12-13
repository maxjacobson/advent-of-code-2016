use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub struct Message {
    matrix: Vec<Vec<char>>,
}

impl Message {
    pub fn new(filename: &str) -> Message {
        let mut f = File::open(filename).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let mut matrix: Vec<Vec<char>> = vec![];

        for line in s.lines() {
            matrix.push(line.chars().collect());
        }

        Message { matrix: matrix }
    }

    pub fn message(&self) -> String {
        let mut result = vec![];
        for x in 0..(self.matrix.iter().nth(0).unwrap().len()) {
            let mut col = vec![];
            for y in 0..self.matrix.len() {
                col.push(self.matrix[y][x]);
            }
            let most = self.most_common(col);
            result.push(most);
        }
        let final_result: String = result.iter().map(|c| *c).collect();
        final_result
    }

    fn most_common(&self, group: Vec<char>) -> char {
        let mut character_counts: HashMap<char, u32> = HashMap::new();
        for c in group {
            let count = character_counts.entry(c).or_insert(0);
            *count += 1;
        }

        let mut character_counts_vec: Vec<(&char, &u32)> = character_counts.iter().collect();
        character_counts_vec.sort_by(|a, b| a.1.cmp(b.1));

        *character_counts_vec.iter().nth(0).unwrap().0
    }
}
