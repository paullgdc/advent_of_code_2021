#![feature(array_windows)]
#![feature(let_else)]

use std::{collections::HashMap, io::BufRead};

fn parse_input() -> (Vec<char>, Vec<([char; 2], char)>) {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let start = lines.next().unwrap().trim().chars().collect();
    lines.next().unwrap();

    let combinations = lines
        .map(|l| {
            let (from, to) = l.split_once("->").unwrap();

            let mut from = from.trim().chars();
            let mut to = to.trim().chars();
            (
                [from.next().unwrap(), from.next().unwrap()],
                to.next().unwrap(),
            )
        })
        .collect();

    (start, combinations)
}

fn count_new_pairs(
    pair_counts: &HashMap<[char; 2], u64>,
    next_pair_count: &mut HashMap<[char; 2], u64>,
    combinatiosn: &HashMap<[char; 2], char>,
) {
    for (pair, count) in pair_counts {
        let Some(&middle) = combinatiosn.get(pair) else {
            *next_pair_count.entry(*pair).or_insert(0) += *count;
            continue;
        };

        *next_pair_count.entry([pair[0], middle]).or_insert(0) += *count;
        *next_pair_count.entry([middle, pair[1]]).or_insert(0) += *count;
    }
}

fn max_min_chars(rounds: usize, start: &[char], combinations: &[([char; 2], char)]) -> u64 {
    let combinations: HashMap<_, _> = combinations.into_iter().copied().collect();
    let mut pair_count = start
        .array_windows::<2>()
        .fold(HashMap::new(), |mut h, &p| {
            *h.entry(p).or_insert(0_u64) += 1;
            h
        });
    let mut next_count = HashMap::new();

    for _ in 0..rounds {
        next_count.clear();
        count_new_pairs(&pair_count, &mut next_count, &combinations);
        std::mem::swap(&mut next_count, &mut pair_count);
    }
    let mut counts: HashMap<_, u64> = HashMap::new();
    for ([_, e], c) in pair_count {
        *counts.entry(e).or_default() += c;
    }
    *counts.get_mut(&start[0]).unwrap() += 1;

    let max = counts.iter().max_by_key(|(_, count)| *count).unwrap().1;
    let min = counts.iter().min_by_key(|(_, count)| *count).unwrap().1;

    max - min
}

fn step_1(start: &[char], combinations: &[([char; 2], char)]) {
    println!("Second step: {}", max_min_chars(10, start, combinations));
}

fn step_2(start: &[char], combinations: &[([char; 2], char)]) {
    println!("Second step: {}", max_min_chars(40, start, combinations));
}

fn main() {
    let (start, combinations) = parse_input();
    step_1(&start, &combinations);
    step_2(&start, &combinations);
}
