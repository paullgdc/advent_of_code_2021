#![feature(let_else)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::Read;

use arrayvec::ArrayVec;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Amphib {
    A,
    B,
    C,
    D,
}

impl Amphib {
    fn to_str(&self) -> &'static str {
        use Amphib::*;
        match self {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
        }
    }

    fn to_bin_nb(&self) -> u8 {
        use Amphib::*;
        match self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }

    fn to_points(&self) -> u64 {
        10_u64.pow(self.to_bin_nb() as u32)
    }
}

const CORRIDOR_LENGTH: usize = 11;
const NB_OF_BINS: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State<const BIN_DEPTH: usize> {
    corridor: [Option<Amphib>; CORRIDOR_LENGTH],
    bins: [ArrayVec<Amphib, BIN_DEPTH>; 4],
}

impl<const BIN_DEPTH: usize> std::fmt::Debug for State<BIN_DEPTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;
        for pos in &self.corridor {
            f.write_str(match pos {
                None => ".",
                Some(a) => a.to_str(),
            })?;
        }
        f.write_str("\n")?;
        for i in 0..BIN_DEPTH {
            f.write_str("  ")?;
            for s in &self.bins {
                f.write_str(match s.get( BIN_DEPTH - 1 - i) {
                    None => ".",
                    Some(a) => a.to_str(),
                })?;
                f.write_str(" ")?;
            }
            f.write_str(" \n")?;
        }
        Ok(())
    }
}

fn parse_input<const BIN_DEPTH: usize>(input: &str) -> State<BIN_DEPTH> {
    fn parse_amphib(s: &str) -> Amphib {
        match s {
            "A" => Amphib::A,
            "B" => Amphib::B,
            "C" => Amphib::C,
            "D" => Amphib::D,
            _ => panic!(),
        }
    }
    let mut state = State {
        corridor: Default::default(),
        bins: Default::default(),
    };
    for l in input.lines().skip(2) {
        let l = l.trim().trim_matches('#');
        if l.is_empty() {
            break;
        }
        // dbg!(l);
        for (symb, stack) in l.split("#").zip(&mut state.bins) {
            stack.push(parse_amphib(symb));
        }
    }
    for s in &mut state.bins {
        s.reverse();
    }
    state
}

fn is_state_final<const BIN_DEPTH: usize>(state: &State<BIN_DEPTH>) -> bool {
    state
        .bins
        .iter()
        .enumerate()
        .all(|(i, b)| b.is_full() && b.iter().all(|a| a.to_bin_nb() == i as u8))
}

fn bin_to_pos(bin_nb: u8) -> u8 {
    (2 + bin_nb * 2) as u8
}

fn can_move_to_pos<const BIN_DEPTH: usize>(
    state: &State<BIN_DEPTH>,
    from: u8,
    amphib: &Amphib,
    to: u8,
) -> Option<u64> {
    // let amphib = state.corridor[from as usize].unwrap();
    let (mut start, mut end) = (from.min(to) as usize, from.max(to) as usize);
    if start == end {
        return None;
    }
    if from == start as u8 {
        start += 1;
    } else {
        end -= 1;
    };
    if state.corridor[start..=end].iter().all(Option::is_none) {
        Some((end - start + 1) as u64 * amphib.to_points())
    } else {
        None
    }
}

fn can_move_to_bin<const BIN_DEPTH: usize>(state: &State<BIN_DEPTH>, from: u8) -> Option<u64> {
    let amphib = state.corridor[from as usize].unwrap();
    let bin = &state.bins[amphib.to_bin_nb() as usize];
    if bin.iter().any(|a| a != &amphib) {
        return None;
    }
    Some(
        can_move_to_pos(state, from, &amphib, bin_to_pos(amphib.to_bin_nb()))?
            + (BIN_DEPTH as u64 - bin.len() as u64) * amphib.to_points(),
    )
}

fn can_get_out_of_bin<const BIN_DEPTH: usize>(state: &State<BIN_DEPTH>, from_bin_idx: u8) -> bool {
    let from_bin = state.bins[from_bin_idx as usize];
    from_bin.iter().any(|a| a.to_bin_nb() != from_bin_idx)
}

fn get_out_of_bin<const BIN_DEPTH: usize>(
    state: &State<BIN_DEPTH>,
    from_bin_idx: u8,
    to_pos: u8,
) -> Option<u64> {
    let amphib = state.bins[from_bin_idx as usize].last().unwrap();
    Some(
        (BIN_DEPTH as u64 + 1 - state.bins[from_bin_idx as usize].len() as u64)
            * amphib.to_points()
            + can_move_to_pos(state, bin_to_pos(from_bin_idx), amphib, to_pos)?,
    )
}

fn is_bin_pos(pos: u8) -> bool {
    [2, 4, 6, 8].iter().any(|p| pos == *p)
}

fn next_states<const BIN_DEPTH: usize>(
    current: &(u64, State<BIN_DEPTH>),
    states: &mut BinaryHeap<Reverse<(u64, State<BIN_DEPTH>)>>,
    visited: &mut HashSet<State<BIN_DEPTH>>,
) {
    if visited.contains(&current.1) {
        return;
    }
    visited.insert(current.1);
    for (pos, a) in current.1.corridor.iter().enumerate() {
        // For amphibians in corridor
        let Some(a) = a else {
            continue;
        };
        // Try to move to it's own bin
        let Some(cost) = can_move_to_bin(&current.1, pos as u8) else {
            continue;
        };
        let mut new_state = current.1.clone();
        new_state.corridor[pos] = None;
        new_state.bins[a.to_bin_nb() as usize].push(*a);
        if visited.contains(&new_state) {
            continue;
        }
        states.push(Reverse((current.0 + cost, new_state)));
    }
    for bin_idx in 0..(NB_OF_BINS as u8) {
        if !can_get_out_of_bin(&current.1, bin_idx) {
            continue;
        }
        for i in 0..(CORRIDOR_LENGTH as u8) {
            if is_bin_pos(i) {
                continue;
            }
            // Try to out of bin
            let Some(cost) = get_out_of_bin(&current.1, bin_idx, i) else {
                continue;
            };
            let mut new_state = current.1.clone();
            let amphib = new_state.bins[bin_idx as usize].pop().unwrap();
            new_state.corridor[i as usize] = Some(amphib);
            if visited.contains(&new_state) {
                continue;
            }
            states.push(Reverse((current.0 + cost, new_state)));
        }
    }
}

fn find_lowest_cost<const BIN_DEPTH: usize>(start: &State<BIN_DEPTH>) -> u64 {
    let mut states = BinaryHeap::<Reverse<(u64, State<BIN_DEPTH>)>>::new();
    let mut visited = HashSet::new();
    states.push(Reverse((0, *start)));
    let cost = loop {
        let Reverse(state) = states.pop().unwrap();
        if is_state_final(&state.1) {
            break state.0;
        }
        next_states(&state, &mut states, &mut visited);
    };
    cost
}

fn step_1(start: &State<2>) -> isize {
    find_lowest_cost(start) as isize
}

fn insert_step_2(start_state: &State<2>) -> State<4> {
    let mut step_2_start_state = State {
        corridor: Default::default(),
        bins: Default::default(),
    };
    use Amphib::*;
    let bin_extension = [[D, D], [B, C], [A, B], [C, A]];
    for ((input_bin, step_2_bin), extension) in start_state
        .bins
        .iter()
        .zip(&mut step_2_start_state.bins)
        .zip(bin_extension)
    {
        step_2_bin.push(input_bin[0]);
        for a in extension {
            step_2_bin.push(a);
        }
        step_2_bin.push(input_bin[1]);
    }

    step_2_start_state
}

fn step_2(start_state: &State<2>) -> isize {
    let start_state = insert_step_2(start_state);
    find_lowest_cost(&start_state) as isize
}

fn main() {
    let mut input = Vec::new();
    std::io::stdin().lock().read_to_end(&mut input).unwrap();
    let input = std::str::from_utf8(&input).unwrap();
    let start_state = parse_input::<2>(&input);
    println!("Step 1: {}", step_1(&start_state));
    println!("Step 2: {}", step_2(&start_state));
}
