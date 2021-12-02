use std::io::BufRead;

fn parse_input() -> Vec<i32> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().trim().parse().unwrap())
        .collect()
}

fn number_of_increases(v: &[i32]) -> usize {
    v.windows(2)
        .map(|window| if window[1] > window[0] { 1 } else { 0 })
        .sum()
}

fn rolling_sum_3(v: &[i32]) -> Vec<i32> {
    v.windows(3)
        .map(|window| window.into_iter().sum())
        .collect()
}

fn main() {
    let input = parse_input();

    let first_step_solution = number_of_increases(&input);
    println!("First step solution: {}", first_step_solution);

    let rolled_3 = rolling_sum_3(&input);
    let second_step_solution = number_of_increases(&rolled_3);
    println!("Second step solution: {}", second_step_solution);
}
