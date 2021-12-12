#![feature(let_else)]
use std::io::BufRead;

use matrix::Matrix;

type Grid = Matrix<u8>;

fn parse_input() -> Matrix<u8> {
    let mut matrix = Matrix::new();
    std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .for_each(|l| {
            matrix
                .next_row()
                .from_iter(l.trim().chars().map(|c| c.to_digit(10).unwrap() as u8))
                .finish();
        });
    matrix
}

fn step(g: &mut Grid) -> usize {
    let mut activated: Matrix<bool> = Matrix::default_with_size(g.dims());
    let mut activations = Vec::new();
    for v in g.iter_mut() {
        *v += 1;
    }

    for (i, j) in g.iter_coords() {
        if *g.get(i, j).unwrap() > 9 {
            *activated.get_mut(i, j).unwrap() = true;
            activations.push((i, j));
        }
    }
    loop {
        let Some((i, j)) = activations.pop() else { break };
        for neigh in g.neighbors_diag_pos(i, j) {
            let neigh_value = g.get_mut(neigh.0, neigh.1).unwrap();
            if *activated.get(neigh.0, neigh.1).unwrap() {
                continue;
            }
            *neigh_value += 1;
            if *neigh_value > 9 {
                *activated.get_mut(neigh.0, neigh.1).unwrap() = true;
                activations.push(neigh);
            }
        }
    }

    let mut number_of_activations = 0;
    for (i, j) in g.iter_coords() {
        let v = g.get_mut(i, j).unwrap();
        if *v > 9 {
            *v = 0;
            number_of_activations += 1;
        }
    }
    number_of_activations
}

fn step_1(mut grid: Grid) {
    let mut total_activations = 0;
    for _ in 0..100 {
        total_activations += step(&mut grid);
    }
    println!("First step result: {}", total_activations)
}

fn step_2(mut grid: Grid) {
    let mut step_nb = 0;
    loop {
        let activations = step(&mut grid);
        step_nb += 1;
        if activations == grid.dims().0 * grid.dims().1 {
            break;
        }
    }
    println!("Second step result: {}", step_nb)
}

fn main() {
    let grid = parse_input();
    step_1(grid.clone());
    step_2(grid);
}
