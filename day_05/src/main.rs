use std::{collections::HashMap, io::BufRead};

fn parse_input() -> Vec<Line> {
    fn parse_point(point: &str) -> Point {
        let (x, y) = point.trim().split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
    std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let (start, end) = l.split_once("->").unwrap();
            Line {
                start: parse_point(start),
                end: parse_point(end),
            }
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

struct Line {
    start: Point,
    end: Point,
}

fn coordinate_delta(start: i16, end: i16) -> i16 {
    use std::cmp::Ordering::*;
    match start.cmp(&end) {
        Equal => 0,
        Less => 1,
        Greater => -1,
    }
}

impl Line {
    fn point_range<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        let mut cur_pos = self.start;
        let mut end = false;
        std::iter::from_fn(move || {
            if cur_pos == self.end {
                if end {
                    return None;
                }
                end = true;
                return Some(cur_pos);
            }
            let pos = cur_pos.clone();
            cur_pos.x += coordinate_delta(self.start.x, self.end.x);
            cur_pos.y += coordinate_delta(self.start.y, self.end.y);
            Some(pos)
        })
    }
}

fn step_1(lines: &[Line]) {
    let mut point_counts = HashMap::new();
    for line in lines {
        if  line.start.x != line.end.x && line.start.y != line.end.y {
            continue;
        }

        for p in line.point_range() {
            *point_counts.entry(p).or_insert(0) += 1;
        }
    }
    let high_wind = point_counts.values().filter(|&&v| v > 1).count();
    println!("First step solution: {}", high_wind);
}


fn step_2(lines: &[Line]) {
    let mut point_counts = HashMap::new();
    for line in lines {
        for p in line.point_range() {
            *point_counts.entry(p).or_insert(0) += 1;
        }
    }
    let high_wind = point_counts.values().filter(|&&v| v > 1).count();
    println!("Second step solution: {}", high_wind);
}

fn main() {
    let lines = parse_input();

    step_1(&lines);
    step_2(&lines);
}
