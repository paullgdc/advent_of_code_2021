#![feature(drain_filter)]

use std::{collections::HashMap, io::BufRead};

fn parse_input() -> (Vec<u8>, Vec<Grid>) {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let draws: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    assert_eq!(lines.next().unwrap().unwrap().trim(), "");
    let mut grids = Vec::new();
    let mut current_grid = Grid::new();
    let mut j = 0;
    for line in lines {
        let line = line.unwrap();
        if line.trim() == "" {
            grids.push(current_grid);
            current_grid = Grid::new();
            j = 0;
            continue;
        }
        for (i, number) in line
            .trim()
            .split(" ")
            .filter(|c| !c.trim().is_empty())
            .enumerate()
        {
            current_grid
                .numbers
                .insert(number.parse().unwrap(), (i as u8, j, false));
        }
        j += 1;
    }
    grids.push(current_grid);
    (draws, grids)
}

#[derive(Clone, Debug)]
struct Grid {
    numbers: HashMap<u8, (u8, u8, bool)>,
    count_row: [u8; 5],
    count_col: [u8; 5],
    win: bool,
}

impl Grid {
    fn new() -> Self {
        Self {
            numbers: HashMap::with_capacity(25),
            count_row: [0; 5],
            count_col: [0; 5],
            win: false,
        }
    }

    fn update_draw(&mut self, draw: u8) -> bool {
        let (i, j, drawn) = match self.numbers.get_mut(&draw) {
            None => return false,
            Some(n) => (n),
        };
        *drawn = true;
        self.count_row[*j as usize] += 1;
        self.count_col[*i as usize] += 1;

        let (i, j) = (*i, *j);
        self.win = self.has_won(i, j);
        self.win
    }

    fn has_won(&self, i: u8, j: u8) -> bool {
        self.count_row[j as usize] == 5 || self.count_col[i as usize] == 5
    }

    fn points_left(&self) -> u64 {
        self.numbers
            .iter()
            .filter(|(_, (_, _, drawn))| !drawn)
            .map(|(v, _)| (*v as u64))
            .sum::<u64>()
    }
}

fn first_step(draws: &[u8], mut grids: Vec<Grid>) {
    let mut last_drawn = 0;
    let mut winning_grid = 0;
    'win: for draw in draws.iter() {
        for (i, grid) in grids.iter_mut().enumerate() {
            let has_won = grid.update_draw(*draw);
            if has_won {
                winning_grid = i;
                last_drawn = *draw;
                break 'win;
            }
        }
    }
    let result: u64 = (last_drawn as u64) * grids[winning_grid].points_left();
    println!("First step result: {}", result);
}

fn second_step(draws: &[u8], grids: Vec<Grid>) {
    let mut not_won_grids: Vec<_> = grids;
    let mut last_draw = 0;
    let mut last_winner = None;
    for draw in draws.iter() {
        match not_won_grids
            .drain_filter(|grid| grid.update_draw(*draw))
            .last()
        {
            Some(g) => {
                if not_won_grids.is_empty() {
                    last_winner = Some(g);
                    last_draw = *draw;
                    break;
                }
            }
            None => {}
        }
    }
    let result = last_winner.unwrap().points_left() * last_draw as u64;
    println!("Second step result: {}", result);
}

fn main() {
    let (draws, grids) = parse_input();

    first_step(&draws, grids.clone());

    second_step(&draws, grids.clone())
}
