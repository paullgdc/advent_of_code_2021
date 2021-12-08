use arrayvec::ArrayVec;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Digit(u8);

impl Digit {
    fn from_char(c: char) -> Self {
        Self(match c {
            'a' => 1 << 1,
            'b' => 1 << 2,
            'c' => 1 << 3,
            'd' => 1 << 4,
            'e' => 1 << 5,
            'f' => 1 << 6,
            'g' => 1 << 7,
            _ => panic!(),
        })
    }

    fn powered_segments(&self) -> u8 {
        self.0.count_ones() as u8
    }

    fn difference(&self, other: &Self) -> Self {
        Self(self.0 & !(self.0 & other.0))
    }

    fn contains(&self, other: &Self) -> bool {
        (self.0 & other.0) == other.0
    }

    fn is_unique(&self) -> bool {
        self.powered_segments() == 2
            || self.powered_segments() == 4
            || self.powered_segments() == 3
            || self.powered_segments() == 7
    }
}

struct Sequence {
    digits: [Digit; 10],
    solution: [Digit; 4],
}

fn parse_digit(s: &str) -> Digit {
    let mut d = Digit(0);
    for c in s.chars() {
        d.0 ^= Digit::from_char(c).0;
    }
    d
}

fn parse_input() -> Vec<Sequence> {
    std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let (digits, solution) = l.split_once('|').unwrap();
            let digits = digits
                .trim()
                .split_ascii_whitespace()
                .map(parse_digit)
                .collect::<ArrayVec<_, 10>>()
                .to_array();

            let solution = solution
                .trim()
                .split_ascii_whitespace()
                .map(parse_digit)
                .collect::<ArrayVec<_, 4>>()
                .to_array();

            Sequence { digits, solution }
        })
        .collect()
}

fn first_step(input: &[Sequence]) {
    let result = input
        .iter()
        .flat_map(|s| s.solution.iter())
        .filter(|&d| d.is_unique())
        .map(|x| x)
        .count();
    println!("First step result: {}", result);
}

fn solve_mapping(s: &Sequence) -> [Digit; 10] {
    let mut digits: ArrayVec<Digit, 10> = s.digits.iter().cloned().collect();
    let mut mapping: [Option<Digit>; 10] = Default::default();

    mapping[1] = digits.drain_filter(|d| d.powered_segments() == 2).last();
    mapping[4] = digits.drain_filter(|d| d.powered_segments() == 4).last();
    mapping[7] = digits.drain_filter(|d| d.powered_segments() == 3).last();
    mapping[8] = digits.drain_filter(|d| d.powered_segments() == 7).last();

    mapping[3] = digits
        .drain_filter(|d| {
            d.powered_segments() == 5 && d.difference(&mapping[7].unwrap()).powered_segments() == 2
        })
        .last();

    let b = mapping[4].unwrap().difference(&mapping[3].unwrap());

    mapping[2] = digits
        .drain_filter(|d| d.powered_segments() == 5 && !d.contains(&b))
        .last();

    mapping[5] = digits.drain_filter(|d| d.powered_segments() == 5).last();

    let e = mapping[2].unwrap().difference(&mapping[3].unwrap()).clone();
    let d = mapping[4]
        .unwrap()
        .difference(&mapping[1].unwrap())
        .difference(&b);

    mapping[0] = digits
        .drain_filter(|di| di.powered_segments() == 6 && !di.contains(&d))
        .last();

    mapping[6] = digits
        .drain_filter(|di| di.powered_segments() == 6 && di.contains(&e))
        .last();

    mapping[9] = digits.pop();

    mapping
        .into_iter()
        .map(Option::unwrap)
        .collect::<ArrayVec<_, 10>>()
        .to_array()
}

fn apply_mapping_to_solution(seq: &Sequence, mapping: &[Digit; 10]) -> u64 {
    let mut sol = 0;
    for d in seq.solution.iter() {
        sol *= 10;
        let n = mapping
            .iter()
            .enumerate()
            .find(|(_, mapped_d)| *mapped_d == d)
            .unwrap()
            .0;
        sol += n as u64;
    }
    sol
}

fn second_step(input: &[Sequence]) {
    let mut sum = 0;
    for seq in input {
        let mapping = solve_mapping(&seq);
        sum += apply_mapping_to_solution(seq, &mapping);
    }
    println!("Second step result: {}", sum);
}

fn main() {
    let input = parse_input();

    first_step(&input);
    second_step(&input);
}
