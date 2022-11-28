#![feature(let_else)]
use std::io::Read;

type Grid = matrix::Matrix<Case>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Case(Option<Direction>);

impl std::fmt::Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.0 {
            None => ".",
            Some(Direction::East) => ">",
            Some(Direction::South) => "v",
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    East,
    South,
}

impl Direction {
    #[inline(always)]
    fn next_pos(&self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Direction::East => (i + 1, j),
            Direction::South => (i, j + 1),
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let mut g = Grid::new();
    for l in input.lines() {
        g.next_row()
            .from_iter(l.as_bytes().iter().map(|c| {
                Case(match *c {
                    b'>' => Some(Direction::East),
                    b'v' => Some(Direction::South),
                    b'.' => None,
                    _ => panic!(),
                })
            }))
            .finish();
    }
    g
}

fn perform_step(from: &Grid, to: &mut Grid, dir: Direction) -> bool {
    let mut change = false;
    assert_eq!(from.dims(), to.dims());
    for (i, j) in from.iter_coords() {
        match from.get(i, j).unwrap() {
            Case(Some(c)) if c != &dir => {
                *to.get_mut(i, j).unwrap() = Case(Some(*c));
                continue;
            }
            Case(None) => continue,
            _ => {}
        }
        let next = dir.next_pos(i, j);
        let next = (next.0 % from.dims().0, next.1 % from.dims().1);
        if let Case(None) = from.get(next.0, next.1).unwrap() {
            // dbg!((i, j), next);
            *to.get_mut(next.0, next.1).unwrap() = Case(Some(dir));
            // *to.get_mut(i, j).unwrap() = Case(None);
            change = true;
        } else {
            *to.get_mut(i, j).unwrap() = Case(Some(dir));
        }
    }
    change
}

fn reset(g: &mut Grid) {
    for e in g.iter_mut() {
        *e = Case(None);
    }
}

fn step_1(g: &Grid) -> isize {
    let mut current_grid = g.clone();
    let mut next_grid = Grid::default_with_size(current_grid.dims());
    let mut step = 0;
    // println!("\n{}\n{}", current_grid, next_grid);
    loop {
        reset(&mut next_grid);
        let change_east = perform_step(&current_grid, &mut next_grid, Direction::East);
        std::mem::swap(&mut current_grid, &mut next_grid);
        // if !change {
        //     break;
        // }
        // println!("\n{}", current_grid);

        reset(&mut next_grid);
        let change_south = perform_step(&current_grid, &mut next_grid, Direction::South);
        std::mem::swap(&mut current_grid, &mut next_grid);
        step += 1;

        if !(change_east || change_south) {
            break;
        }
        // println!("\n{}", current_grid);
    }
    step
}

fn main() {
    let mut input = Vec::new();
    std::io::stdin().lock().read_to_end(&mut input).unwrap();
    let input = std::str::from_utf8(&input).unwrap();

    let g = parse_input(input);

    println!("First step: {}", step_1(&g));
}
