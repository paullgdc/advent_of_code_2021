use std::io::BufRead;

fn parse_input() -> (Vec<u16>, u8) {
    let mut number_of_digits = None;
    (
        std::io::stdin()
            .lock()
            .lines()
            .map(|l| {
                let line = l.unwrap();
                let mut n = 0;
                let mut num_of_digits = 0;
                for digit in line.trim().chars() {
                    n <<= 1;
                    match digit {
                        '0' => {}
                        '1' => n += 1,
                        _ => panic!("Unexpected char {}", digit),
                    }
                    num_of_digits += 1;
                }
                match number_of_digits {
                    Some(num) => assert_eq!(num, num_of_digits),
                    None => number_of_digits = Some(num_of_digits),
                };
                n
            })
            .collect(),
        number_of_digits.unwrap(),
    )
}

fn most_frequent_digit(power_of_2: u8, numbers: &[u16]) -> Option<u16> {
    let mut count_of_0 = 0;
    let mask = 1 << power_of_2;
    for number in numbers {
        if number & mask == 0 {
            count_of_0 += 1;
        }
    }
    use std::cmp::Ordering::*;
    match count_of_0.cmp(&(numbers.len() / 2)) {
        Equal => None,
        Greater => Some(0),
        Less => Some(1)
    }
}

fn bit_at_pos(n: u16, power_of_2: u8) -> u16 {
    (n & (1 << power_of_2)) >> power_of_2
}

fn first_step(numbers: &[u16], num_of_digits: u8) {
    let mut gamma_rate = 0;
    for power_of_2 in 0..num_of_digits {
        gamma_rate += most_frequent_digit(power_of_2, numbers).unwrap_or(1) << power_of_2;
    }
    let epsilon_mask = (1 << num_of_digits) - 1;
    let epsilon_rate = (gamma_rate ^ epsilon_mask) & epsilon_mask;

    let result = epsilon_rate as u64 * gamma_rate as u64;

    println!("First step result: {}", result);
}

fn second_step(numbers: &[u16], num_of_digits: u8) {
    let mut most_frequent_set = numbers.to_owned();
    for power_of_2 in (0..num_of_digits).rev() {
        if most_frequent_set.len() == 1 {
            break;
        }
        let most_frequent = most_frequent_digit(power_of_2, &most_frequent_set).unwrap_or(1);
        most_frequent_set.retain(|&n| bit_at_pos(n, power_of_2) == most_frequent);
    }

    let mut least_frequent_set = numbers.to_owned();
    for power_of_2 in (0..num_of_digits).rev() {
        if least_frequent_set.len() == 1 {
            break;
        }
        let least_frequent = most_frequent_digit(power_of_2, &least_frequent_set).unwrap_or(1) ^ 1;
        least_frequent_set.retain(|&n| bit_at_pos(n, power_of_2) == least_frequent);
    }
    let oxy_gen_rating = most_frequent_set[0];
    let c02_gen_rating = least_frequent_set[0];
    let result = oxy_gen_rating as u64 * c02_gen_rating as u64;
    println!("Second step result: {}", result);
}

fn main() {
    let input = parse_input();
    first_step(&input.0, input.1);
    second_step(&input.0, input.1);
}
