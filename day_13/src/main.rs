use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

type Point = (usize, usize);

fn parse_input() -> (Vec<Point>, Vec<Fold>) {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let positions = lines
        .by_ref()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| {
            let (left, right) = l.trim().split_once(',').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    let folds: Vec<_> = lines
        .map(|l| {
            let (f, pos) = l
                .trim()
                .strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap();
            match f {
                "x" => Fold::X(pos.parse().unwrap()),
                "y" => Fold::Y(pos.parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect();

    (positions, folds)
}

fn fold_coord(coord: usize, fold: usize) -> usize {
    if coord < fold {
        coord
    } else {
        (2 * fold) - coord
    }
}

fn apply_fold(positions: &[Point], fold: Fold) -> Vec<Point> {
    let mut new_points: Vec<_> = positions
        .iter()
        .map(|&(x, y)| match fold {
            Fold::X(f) => (fold_coord(x, f), y),
            Fold::Y(f) => (x, fold_coord(y, f)),
        })
        .collect();

    new_points.sort();
    new_points.dedup();

    new_points
}

fn to_grid(positions: &[Point]) -> matrix::Matrix<char> {
    let dim_x = positions.iter().max_by_key(|(x, _)| *x).unwrap().0;
    let dim_y = positions.iter().max_by_key(|(_, y)| *y).unwrap().1;

    let mut mat = matrix::Matrix::new_with_elem((dim_x + 1, dim_y + 1), ' ');
    for &(x, y) in positions {
        *mat.get_mut(x, y).unwrap() = '*';
    }
    mat
}

fn step_1(positions: &[Point], folds: &[Fold]) {
    let first = apply_fold(positions, folds[0]);

    let solution = first.len();

    println!("First step: {}", solution);
}

fn step_2(positions: &[Point], folds: &[Fold]) {
    let mut positions = positions;
    let mut buff;
    for &f in folds {
        buff = apply_fold(positions, f);
        positions = &buff;
    }

    let grid = to_grid(positions);

    println!("Second step: \n{}", grid);
}

fn main() {
    let (positions, folds) = parse_input();
    step_1(&positions, &folds);
    step_2(&positions, &folds);
}
