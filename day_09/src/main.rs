use matrix::Matrix;
use std::{io::BufRead, collections::HashSet};

type Grid = Matrix<u8>;

fn parse_input() -> Grid {
    let mut mat = Matrix::new();
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

// fn neightbors(g: &Grid, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
//     let (left, right, bot, up);
//     if i == 0 {
//         left = None;
//     } else {
//         left = Some((i - 1, j));
//     }
//     if i >=
// }

fn step_1(g: &Grid) {
    let mut local_minimas = 0;
    for (i, j) in g.iter_coords() {
        // dbg!(g.dims(), i, j);
        let mut greater_neighbors = 0;
        let mut neighbors_count = 0;
        for (_, neigh) in g.neighbors(i, j) {
            neighbors_count += 1;
            if neigh > g.get(i, j).unwrap() {
                greater_neighbors += 1;
            }
        }
        if greater_neighbors == neighbors_count {
            local_minimas = local_minimas + 1 + (*g.get(i, j).unwrap() as u64);
        }
    }
    println!("First step result: {}", local_minimas);
}

fn step_2(g: &Grid) {
    let mut biggest_bassins = arrayvec::ArrayVec::<u64, 3>::new();
    for (i, j) in g.iter_coords() {
        let value = *g.get(i, j).unwrap();

        let mut greater_neighbors = 0;
        let mut neighbors_count = 0;
        for (_, neigh) in g.neighbors(i, j) {
            neighbors_count += 1;
            if *neigh > value {
                greater_neighbors += 1;
            }
        }
        if greater_neighbors != neighbors_count {
            continue;
        }
        let mut bassin_boundaries = vec![((i, j), value)];
        let mut bassin_visited = HashSet::new();
        loop {
            let next = match bassin_boundaries.pop() {
                Some(n) => n,
                None => break,
            };
            bassin_visited.insert(next.0);
            for ((ni, nj), &nval) in g.neighbors(next.0.0, next.0.1) {
                if nval != 9 && nval > next.1 && !bassin_visited.contains(&(ni, nj)) {
                    bassin_boundaries.push(((ni, nj), nval));
                }
            }
        }
        if biggest_bassins.len() == 3 {
            let biggest_bassins_min = biggest_bassins.iter_mut().min().unwrap();
            if *biggest_bassins_min < bassin_visited.len() as u64 {
                *biggest_bassins_min = bassin_visited.len() as u64;
            }
        } else {
            biggest_bassins.push(bassin_visited.len() as u64);
        }
    }
    println!("Second step result: {}", biggest_bassins.iter().copied().product::<u64>());
}

fn main() {
    let grid = parse_input();
    // dbg!(grid);
    step_1(&grid);
    step_2(&grid);
}
