#![feature(let_else)]
#![feature(map_first_last)]
use arrayvec::ArrayVec;
use std::{
    collections::{BTreeMap, HashMap},
    io::BufRead,
};

fn parse_input() -> [u8; 2] {
    std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let (_player_nb, rest) = l
                .trim()
                .strip_prefix("Player ")
                .unwrap()
                .split_once(" ")
                .unwrap();
            rest.strip_prefix("starting position: ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect::<ArrayVec<_, 2>>()
        .to_array()
}

fn step_1(starting_pos: [u8; 2]) {
    let mut positions: Vec<_> = starting_pos.iter().map(|p| (*p, 0_u16)).collect();
    let mut dice: u64 = 1;
    'outer: loop {
        for (pos, score) in &mut positions {
            let advance = ((dice * 3 + 3) % 10) as u8;
            dice += 3;
            *pos = (*pos + advance - 1) % 10 + 1;
            *score += *pos as u16;
            if *score >= 1000 {
                break 'outer;
            }
        }
    }
    let result = (dice - 1) * positions.iter().map(|(_, s)| *s).min().unwrap() as u64;
    println!("Step 1: {}", result);
}

#[allow(dead_code)]
fn multinomial() {
    let mut counts = HashMap::new();
    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                *counts.entry(i + j + k).or_insert(0) += 1;
            }
        }
    }
    dbg!(counts);
}

const MUTINOMIAL_DICES: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, PartialEq, Eq, Ord)]
struct Universe([(u8, u8); 2]);

impl std::cmp::PartialOrd for Universe {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            match (self.0[0].0 + self.0[1].0).cmp(&(other.0[0].0 + other.0[1].0)) {
                std::cmp::Ordering::Equal => self.0.cmp(&other.0),
                other => other,
            },
        )
    }
}

fn advance(roll: u8, p_state: (u8, u8)) -> (u8, u8) {
    let next_pos = (p_state.1 + roll - 1) % 10 + 1;
    let next_score = p_state.0 + next_pos;
    (next_score, next_pos)
}

fn step_2(starting_pos: [u8; 2]) {
    let mut players_wins = [0, 0];
    let mut universes = BTreeMap::new();

    universes.insert(Universe([(0, starting_pos[0]), (0, starting_pos[1])]), 1);
    loop {
        let Some((Universe(universe), count)) = universes.pop_first() else {
            break;
        };
        for (d1_roll, d1_counts) in MUTINOMIAL_DICES {
            let next_p1 = advance(d1_roll, universe[0]);
            if next_p1.0 >= 21 {
                players_wins[0] += d1_counts * count;
                continue;
            }
            for (d2_roll, d2_counts) in MUTINOMIAL_DICES {
                let next_p2 = advance(d2_roll, universe[1]);
                if next_p2.0 >= 21 {
                    players_wins[1] += d1_counts * d2_counts * count;
                    continue;
                }
                let next_universe = [next_p1, next_p2];
                *universes.entry(Universe(next_universe)).or_default() +=
                    d1_counts * d2_counts * count;
            }
        }
    }
    let result = players_wins.iter().copied().max().unwrap();
    println!("Step 2: {}", result);
}

fn main() {
    let start_pos = parse_input();
    step_1(start_pos);
    step_2(start_pos);
}
