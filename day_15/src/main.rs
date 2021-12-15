#![feature(let_else)]
use matrix::Matrix;
use std::{
    collections::{BinaryHeap, HashSet},
    io::BufRead,
};

type Grid = Matrix<u8>;

struct ExtendedGrid(Grid);

impl ExtendedGrid {
    fn dims(&self) -> (usize, usize) {
        (self.0.dims().0 * 5, self.0.dims().1 * 5)
    }

    fn get(&self, i: usize, j: usize) -> Option<u8> {
        if i >= self.dims().0 || j >= self.dims().1 {
            return None;
        }
        let real_i = i % self.0.dims().0;
        let real_j = j % self.0.dims().0;

        let add_i = i / self.0.dims().0;
        let add_j = j / self.0.dims().0;

        self.0
            .get(real_i, real_j)
            .map(|r| (*r - 1 + add_i as u8 + add_j as u8) % 9 + 1)
    }

    fn neighbors_pos(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
        (i > 0)
            .then(|| (i - 1, j))
            .into_iter()
            .chain((i + 1 < self.dims().0).then(|| (i + 1, j)))
            .chain((j > 0).then(|| (i, j - 1)))
            .chain((j + 1 < self.dims().1).then(|| (i, j + 1)))
    }
}

fn parse_input() -> Grid {
    let mut mat = Grid::new();
    std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .for_each(|l| {
            mat.next_row()
                .from_iter(l.trim().chars().map(|c| c.to_digit(10).unwrap() as u8))
                .finish();
        });
    mat
}

fn step_1(g: &Grid) {
    use std::cmp::Reverse;
    let end = (g.dims().0 - 1, g.dims().1 - 1);
    let mut boundary = BinaryHeap::new();
    let mut visited = HashSet::new();
    boundary.push(Reverse((0, (0, 0))));

    let mut min_cost = None;

    'djikstra: loop {
        let Some(Reverse((path_cost, pos))) = boundary.pop() else {
            panic!()
        };
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        for neigh in g.neighbors_pos(pos.0, pos.1) {
            let neigh_cost = *g.get(neigh.0, neigh.1).unwrap() as u64 + path_cost;
            if neigh == end {
                min_cost = Some(neigh_cost);
                break 'djikstra;
            }
            boundary.push(Reverse((neigh_cost, neigh)));
        }
    }
    let min_cost = min_cost.unwrap();
    println!("First step: {}", min_cost);
}

fn step_2(g: &ExtendedGrid) {
    use std::cmp::Reverse;
    let end = (g.dims().0 - 1, g.dims().1 - 1);
    let mut boundary = BinaryHeap::new();
    let mut visited = HashSet::new();
    boundary.push(Reverse((0, (0, 0))));

    let mut min_cost = None;

    'djikstra: loop {
        let Some(Reverse((path_cost, pos))) = boundary.pop() else {
            panic!()
        };
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        for neigh in g.neighbors_pos(pos.0, pos.1) {
            let neigh_cost = g.get(neigh.0, neigh.1).unwrap() as u64 + path_cost;
            if neigh == end {
                min_cost = Some(neigh_cost);
                break 'djikstra;
            }
            boundary.push(Reverse((neigh_cost, neigh)));
        }
    }
    let min_cost = min_cost.unwrap();
    println!("Second step: {}", min_cost);
}

fn main() {
    let grid = parse_input();
    step_1(&grid);
    step_2(&ExtendedGrid(grid));
}
