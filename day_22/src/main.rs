#![feature(bool_to_option)]

use std::{collections::HashMap, io::BufRead};

type Cube = ([(i32, i32); 3]);

fn parse_input(input: &str) -> Vec<(Cube, bool)> {
    fn parse_couple(couple: &str) -> (i32, i32) {
        let (f, s) = couple.split_once('=').unwrap().1.split_once("..").unwrap();
        (f.parse().unwrap(), s.parse().unwrap())
    }

    input
        .lines()
        .map(|l| {
            let (state, coords) = l.trim().split_once(" ").unwrap();

            let split = coords.split(',');
            let x = parse_couple(split.next().unwrap());
            let y = parse_couple(split.next().unwrap());
            let z = parse_couple(split.next().unwrap());
            ([x, y, z], state == "on")
        })
        .collect()
}

fn one_d_intersection(s1: (i32, i32), s2: (i32, i32)) -> Option<(i32, i32)> {
    let (s1, s2) = if s1 < s2 { (s1, s2) } else { (s2, s1) };
    let left = (s2.0 < s1.1).then_some(s2.0)?;
    let right = if s2.1 < s1.1 { s2.1 } else { s1.1 };
    Some((left, right))
}

fn cube_intersection(c1: Cube, c2: Cube) -> Option<Cube> {
    let x = one_d_intersection(c1[0], c2[0])?;
    let y = one_d_intersection(c1[1], c2[1])?;
    let z = one_d_intersection(c1[2], c2[3])?;
    Some([x, y, z])
}

fn split_cube(c1: Cube, c2: Cube) -> (Vec<Cube>, Option<(Cube, Cube)>) {
    let mut splits = Vec::new();
    let (first, second) = if c1[0] < c2[0] { (c1, c2) } else { (c2, c1) };
    let middle = one_d_intersection(first[0], second[0]).unwrap();
    splits.push({
        let mut first_part = first;
        first_part[0].1 = middle.0;
        first_part
    });
    let mut end;
    let mut middle_cube;
    if middle.1 == first[0].1 {
        end = second;
        end[0].0 = middle.1;
    } else {
        end = first;
        end[0].0 = middle.1;
    }
    splits
}

fn step_1(cubes: &[(Cube, bool)]) -> isize {
    let points = HashMap::new();
}

fn main() {
    let cubes = parse_input(
        &std::io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .collect::<String>(),
    );
    println!("{}", step_1(&cubes));
}
