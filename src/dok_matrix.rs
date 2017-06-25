use std::collections::HashMap;
use std::cmp::PartialEq;


#[derive(Deserialize, Serialize, Debug)]
pub struct DokMatrix<T> {
    hash_map: HashMap<(usize, usize), T>,
    rows: usize,
    cols: usize,
    default_value: T,
}

impl<T> DokMatrix<T> {
    pub fn new(rows: usize, cols: usize, default_value: T) -> Self {
        let hash_map: HashMap<(usize, usize), T> = HashMap::new();
        DokMatrix {
            hash_map: hash_map,
            rows: rows,
            cols: cols,
            default_value: default_value
        }
    }

    pub fn set(&mut self, i: usize, j: usize, value: T)
        -> Result<(), String>
        where T: Copy + PartialEq {
        if i < self.rows && j < self.cols {
            if value != self.default_value {
                *self.hash_map.entry((i, j)).or_insert(value) = value;
            }
            Ok(())
        } else {
            Err(format!("insert error ({}, {}), rows: {}, cols: {}", i, j, self.rows, self.cols))
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i < self.rows && j < self.cols {
            Some(self.hash_map.get(&(i, j)).unwrap_or(&self.default_value))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.hash_map.len()
    }
}
