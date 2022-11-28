use std::{io::BufRead, vec};

fn parse_range(r: &str) -> (isize, isize) {
    let (start, end) = r
        .split_once("=")
        .unwrap()
        .1
        .trim()
        .split_once("..")
        .unwrap();
    let (start, end): (isize, isize) = (start.parse().unwrap(), end.parse().unwrap());
    (start.min(end), start.max(end))
}

type Block = ((isize, isize), (isize, isize));

fn parse_input() -> Block {
    let line = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let (x, y) = line
        .trim()
        .strip_prefix("target area: ")
        .unwrap()
        .split_once(",")
        .unwrap();
    (parse_range(x), parse_range(y))
}

fn simulate_x_trajectory(v_zero: isize) -> impl Iterator<Item = isize> {
    let mut pos = 0;
    let mut v = v_zero;
    std::iter::from_fn(move || {
        if v == 0 {
            return None;
        }
        let curr_pos = pos;
        pos += v;
        v = v - 1 * v.signum();
        Some(curr_pos)
    })
}

fn simulate_trajectory(v_zero: (isize, isize), steps: u8) {
    let mut pos = (0, 0);
    let mut v = v_zero;
    for i in 0..steps {
        dbg!(i, pos);
        pos.0 += v.0;
        pos.1 += v.1;
        v.0 = v.0 - 1 * v.0.signum();
        v.1 = v.1 - 1;
    }
}

fn pos_in_block(pos: (isize, isize), b: Block) -> bool {
    b.0 .0 <= pos.0 && pos.0 <= b.0 .1 && b.1 .0 <= pos.1 && pos.1 <= b.1 .1
}

fn step_1(b: Block) {
    let mut acceptable_v_x = Vec::new();
    for v_x in 0..(b.0 .0.abs()) {
        let v_x = v_x * b.0 .0.signum();
        for (step, pos) in simulate_x_trajectory(v_x).enumerate() {
            if b.0 .0 <= pos && pos <= b.0 .1 {
                acceptable_v_x.push((step, v_x));
                break;
            }
        }
    }
    acceptable_v_x.sort_by_key(|(v, s)| *s);
    dbg!(acceptable_v_x);
}

fn main() {
    // let ranges = parse_input();
    // dbg!(ranges);

    // simulate_trajectory((3, 3), 6);
    step_1(((-30, -20), (-5, -10)))
}
