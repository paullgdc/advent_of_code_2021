#![feature(bool_to_option)]
use std::fmt::Debug;

pub struct Matrix<T> {
    dimensions: (usize, usize),
    storage: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new() -> Self {
        Self {
            dimensions: (0, 0),
            storage: Vec::new(),
        }
    }

    pub fn dims(&self) -> (usize, usize) {
        self.dimensions
    }

    pub fn next_row(&mut self) -> MatrixRow<T> {
        MatrixRow {
            mat: self,
            row_size: 0,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i >= self.dimensions.0 {
            return None;
        }
        self.storage.get(i + self.dimensions.0 * j)
    }

    pub fn neighbors_pos(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
        (i > 0)
            .then(|| (i - 1, j))
            .into_iter()
            .chain((i + 1 < self.dimensions.0).then(|| (i + 1, j)))
            .chain((j > 0).then(|| (i, j - 1)))
            .chain((j + 1 < self.dimensions.1).then(|| (i, j + 1)))
    }

    pub fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.neighbors_pos(i, j)
            .map(|(i, j)| ((i, j), self.get(i, j).unwrap()))
    }

    pub fn iter_coords<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        let mut i = 0;
        let mut j = 0;
        std::iter::from_fn(move || {
            while i == self.dimensions.0 && j < self.dimensions.1 {
                i = 0;
                j += 1;
            }
            if j == self.dimensions.1 {
                return None;
            }
            let pos = (i, j);
            i += 1;
            Some(pos)
        })
    }
}

impl<T: Default> Matrix<T> {
    pub fn default_with_size(dimensions: (usize, usize)) -> Self {
        let mut storage = Vec::new();
        storage.resize_with(dimensions.0 * dimensions.1, Default::default);
        Self {
            dimensions,
            storage,
        }
    }
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        if self.dimensions.1 > 0 {
            f.write_str("\n")?;
        }
        for row in 0..self.dimensions.1 {
            f.write_str("  [")?;
            if self.dimensions.0 > 0 {
                f.write_fmt(format_args!("{:?}", self.get(0, row).unwrap()))?;
            }
            for col in 1..self.dimensions.0 {
                f.write_fmt(format_args!(", {:?}", self.get(col, row).unwrap()))?;
            }
            f.write_str("]\n")?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

pub struct MatrixRow<'a, T> {
    mat: &'a mut Matrix<T>,
    row_size: usize,
}

impl<'a, T> MatrixRow<'a, T> {
    pub fn from_iter<U: IntoIterator<Item = T>>(mut self, iter: U) -> Self {
        for i in iter {
            self.row_size += 1;
            self.mat.storage.push(i);
        }
        self
    }

    pub fn finish(self) {
        if self.mat.dimensions.1 == 0 {
            self.mat.dimensions.0 = self.row_size;
        }
        assert_eq!(self.mat.dimensions.0, self.row_size);
        self.mat.dimensions.1 += 1;
    }
}

impl<'a, T> Drop for MatrixRow<'a, T> {
    fn drop(&mut self) {
        self.mat
            .storage
            .truncate(self.mat.dimensions.0 * self.mat.dimensions.1);
    }
}
