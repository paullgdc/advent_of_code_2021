#![feature(let_else)]
#![feature(bool_to_option)]

use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3(i32, i32, i32);

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Clone, Copy, Debug)]
enum Axis {
    XP,
    XM,
    YP,
    YM,
    ZP,
    ZM,
}

impl Axis {
    fn apply(&self, p: Vec3) -> Vec3 {
        use Axis::*;
        match self {
            XP => p,
            XM => Vec3(-p.0, -p.1, p.2),
            YP => Vec3(p.1, -p.0, p.2),
            YM => Vec3(-p.1, p.0, p.2),
            ZP => Vec3(p.2, p.1, -p.0),
            ZM => Vec3(-p.2, p.1, p.0),
        }
    }
    fn values() -> &'static [Self] {
        use Axis::*;
        &[XP, XM, YP, YM, ZP, ZM]
    }
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Q0,
    Q1,
    Q2,
    Q3,
}

impl Rotation {
    fn apply(&self, p: Vec3) -> Vec3 {
        use Rotation::*;
        match self {
            Q0 => p,
            Q1 => Vec3(p.0, p.2, -p.1),
            Q2 => Vec3(p.0, -p.1, -p.2),
            Q3 => Vec3(p.0, -p.2, p.1),
        }
    }
    fn values() -> &'static [Self] {
        use Rotation::*;
        &[Q0, Q1, Q2, Q3]
    }
}

#[derive(Clone, Copy, Debug)]
struct Orientation(Axis, Rotation);

impl Orientation {
    fn apply(&self, p: Vec3) -> Vec3 {
        self.1.apply(self.0.apply(p))
    }

    fn values() -> impl Iterator<Item = Self> {
        Axis::values()
            .iter()
            .flat_map(|&a| Rotation::values().iter().map(move |&r| Self(a, r)))
    }

    fn default() -> Self {
        Self(Axis::XP, Rotation::Q0)
    }
}

#[test]
fn test_rotation_uniques() {
    use std::collections::HashSet;
    let p = Vec3(1, 2, 3);
    let mut s = HashSet::new();
    for rot in Rotation::values().into_iter() {
        for face in Axis::values().into_iter() {
            let point = rot.apply(face.apply(p));
            s.insert(point);
        }
    }
    assert_eq!(s.len(), Axis::values().len() * Rotation::values().len());
}

fn parse_input() -> Vec<Vec<Vec3>> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let mut scanners = Vec::new();
    loop {
        let mut scanner = Vec::new();
        let Some(header) = lines.next() else {
            break
        };
        let _scanner_nb = header
            .trim()
            .strip_prefix("--- scanner ")
            .unwrap()
            .strip_suffix(" ---")
            .unwrap();
        for l in lines.by_ref() {
            if l.trim().is_empty() {
                break;
            }
            let (x, (y, z)) = l
                .trim()
                .split_once(',')
                .and_then(|(f, rest)| Some((f, rest.split_once(',')?)))
                .unwrap();
            scanner.push(Vec3(
                x.parse().unwrap(),
                y.parse().unwrap(),
                z.parse().unwrap(),
            ))
        }
        scanners.push(scanner);
    }

    scanners
}

fn gt_than_0(p: Vec3) -> bool {
    use std::cmp::Ordering::*;
    match p.0.cmp(&0) {
        Less => false,
        Greater => true,
        Equal => match p.1.cmp(&0) {
            Less => false,
            Greater => true,
            Equal => match p.2.cmp(&0) {
                Less => false,
                Greater => true,
                Equal => panic!("points shouldn't be equal"),
            },
        },
    }
}

fn make_oriented(p: Vec3, o: Vec3) -> (Vec3, Vec3) {
    match gt_than_0(o - p) {
        true => (o, p),
        false => (p, o),
    }
}

fn point_offsets_to_pair(points: &[Vec3]) -> HashMap<Vec3, Vec<(usize, usize)>> {
    let mut offsets: HashMap<_, Vec<_>> =
        HashMap::with_capacity((points.len() * (points.len() + 1)) / 2);
    for (pair, offset) in points_offsets(points, Orientation::default()) {
        offsets.entry(offset).or_default().push(pair);
    }
    offsets
}

fn points_offsets<'a>(
    points: &'a [Vec3],
    orientation: Orientation,
) -> impl Iterator<Item = ((usize, usize), Vec3)> + 'a {
    points.iter().enumerate().flat_map(move |(i, &first)| {
        points
            .iter()
            .enumerate()
            .skip(i + 1)
            .map(move |(j, &second)| {
                let first = orientation.apply(first);
                let second = orientation.apply(second);
                let (start, end) = make_oriented(first, second);
                let offset = end - start;
                ((start == first).then_some((i, j)).unwrap_or((j, i)), offset)
            })
    })
}

fn match_scanners(
    scanner_to_search: &[Vec3],
    orientation: Orientation,
    other_scanner_offsets: &HashMap<Vec3, Vec<(usize, usize)>>,
) -> Option<(usize, usize)> {
    let mut matches: HashMap<usize, HashMap<usize, u8>> = HashMap::new();

    for (p_1, off) in points_offsets(scanner_to_search, orientation) {
        if !other_scanner_offsets.contains_key(&off) {
            continue;
        }
        for &p_2 in &other_scanner_offsets[&off] {
            *matches.entry(p_1.0).or_default().entry(p_2.0).or_default() += 1;
        }
    }
    matches
        .iter()
        .filter_map(|(&p, correlations)| Some((p, *correlations.iter().find(|(_, &c)| c >= 11)?.0)))
        .next()
}

fn find_match_with_resolved(
    resolved_scanner: &[Vec3],
    scanners_left: &mut Vec<&Vec<Vec3>>,
    found: &mut Vec<(Vec<Vec3>, Vec3)>,
) {
    let offsets = point_offsets_to_pair(resolved_scanner);
    scanners_left.retain(|points| {
        for orientation in Orientation::values() {
            if let Some(point_match) = match_scanners(points, orientation, &offsets) {
                let pos_1 = orientation.apply(points[point_match.0]);
                let pos_2 = resolved_scanner[point_match.1];
                let ref_coords = pos_2 - pos_1;
                found.push((
                    points
                        .iter()
                        .map(|p| orientation.apply(*p) + ref_coords)
                        .collect(),
                    ref_coords,
                ));
                return false;
            }
        }
        true
    });
}

fn resolve_scanners(scanners: &[Vec<Vec3>]) -> Vec<(Vec<Vec3>, Vec3)> {
    let mut scanners_left: Vec<_> = scanners.iter().skip(1).collect();
    let mut resolved_scanners = vec![(scanners[0].clone(), Vec3(0, 0, 0))];
    let mut last_resolved_pos = 0;

    while !scanners_left.is_empty() {
        let mut found = Vec::new();
        for (resolved_scanner, _) in &resolved_scanners[last_resolved_pos..] {
            find_match_with_resolved(resolved_scanner, &mut scanners_left, &mut found);
        }
        last_resolved_pos = resolved_scanners.len();
        resolved_scanners.extend(found.into_iter());
    }
    resolved_scanners
}

fn step_1(scanners: &[Vec<Vec3>]) {
    let real = resolve_scanners(scanners);
    let unique: HashSet<_> = real.iter().flat_map(|(p, _)| p).collect();
    println!("Step 1: {}", unique.len());
}

fn step_2(scanners: &[Vec<Vec3>]) {
    let real = resolve_scanners(scanners);
    let max_dist = real
        .iter()
        .enumerate()
        .flat_map(|(i, (_, c_1))| {
            real.iter().skip(i + 1).map(|(_, c_2)| {
                let diff = *c_1 - *c_2;
                diff.0.abs() + diff.1.abs() + diff.2.abs()
            })
        })
        .max()
        .unwrap();
    println!("Step 2: {}", max_dist);
}

fn main() {
    let scanners = parse_input();
    step_1(&scanners);
    step_2(&scanners);
}
