use std::io::BufRead;

fn parse_input() -> Vec<u8> {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .collect()
}

fn reproduce(start_days: &[u8], days_to_live: u32) -> u64 {
    let mut bins = [0_u64; 9];
    let mut current_day = 0;
    for day in start_days {
        bins[(*day as usize + current_day) % 9] += 1;
    }

    for _ in 0..days_to_live {
        bins[(current_day + 7) % 9] += bins[current_day];
        current_day = (current_day + 1) % 9;
    }

    bins.iter().sum()
}

fn step_1(start_days: &[u8]) {
    let result = reproduce(start_days, 80);
    println!("First step result: {}", result);
}

fn step_2(start_days: &[u8]) {
    let result = reproduce(start_days, 256);
    println!("Second step result: {}", result);
}

fn main() {
    let start_days = parse_input();
    step_1(&start_days);
    step_2(&start_days);
}
