#![feature(array_windows)]
use std::io::BufRead;

fn parse_input() -> Vec<u16> {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(str::parse::<u16>)
        .map(Result::unwrap)
        .collect()
}

fn step_1(start_positions: &[u16]) {
    let mut positions = start_positions.to_owned();
    positions.sort();

    let median = positions[(positions.len() / 2)];
    let result: i64 = positions
        .iter()
        .map(|p| (*p as i64 - median as i64).abs())
        .sum();
    println!("First step result: min cost {}", result);
}

fn cost_one_direction(
    pos_diff_to_crab_before: i64,
    number_crabs_before: i64,
    sum_distances_crabs_before: i64,
    cost_crab_before: i64,
) -> i64 {
    let added_cost = number_crabs_before
        * ((pos_diff_to_crab_before * (pos_diff_to_crab_before + 1)) / 2)
        + pos_diff_to_crab_before * sum_distances_crabs_before;
    added_cost + cost_crab_before
}

fn pos_cost_both_sides(
    dist_left_crab: i64,
    left_crab: &CrabMoveCost,
    right_crab: &CrabMoveCost,
) -> i64 {
    cost_one_direction(
        dist_left_crab,
        left_crab.crab_idx as i64 + 1,
        left_crab.accumulated_distance,
        left_crab.cost,
    ) + cost_one_direction(
        (right_crab.pos - left_crab.pos) - dist_left_crab,
        right_crab.crab_idx as i64 + 1,
        right_crab.accumulated_distance,
        right_crab.cost,
    )
}

#[derive(Debug)]
struct CrabMoveCost {
    crab_idx: usize,
    pos: i64,
    accumulated_distance: i64,
    cost: i64,
}

fn crab_moving_costs_one_direction(crab_positions: impl Iterator<Item = i64>) -> Vec<CrabMoveCost> {
    let mut crab_positions = crab_positions.enumerate();
    let mut costs = Vec::new();
    let (_, first_crab_pos) = crab_positions.next().unwrap();
    costs.push(CrabMoveCost {
        pos: first_crab_pos,
        accumulated_distance: 0,
        cost: 0,
        crab_idx: 0,
    });

    for (i, pos) in crab_positions {
        let previous_crab = costs.last().unwrap();
        let diff_pos = (pos - previous_crab.pos).abs();
        let cost = cost_one_direction(
            diff_pos,
            i as i64,
            previous_crab.accumulated_distance,
            previous_crab.cost,
        );
        let accumulated_distance = i as i64 * diff_pos + previous_crab.accumulated_distance;
        costs.push(CrabMoveCost {
            pos: pos,
            accumulated_distance,
            cost,
            crab_idx: i,
        })
    }

    costs
}

fn crab_segment_min_cost(left_crab: &CrabMoveCost, right_crab: &CrabMoveCost) -> (i64, i64) {
    if right_crab.pos == left_crab.pos {
        return (left_crab.cost + right_crab.cost, right_crab.pos);
    }
    let l = (right_crab.pos - left_crab.pos) as f64;

    let pos_min_cost =
        l * (right_crab.crab_idx + 1) as f64 - (left_crab.pos - right_crab.pos) as f64 / 2.0;
    let pos_min_cost =
        (pos_min_cost / ((left_crab.pos + right_crab.pos) as f64 + 2.0)).clamp(0.0, l);
    let left = pos_min_cost.trunc() as i64;
    let right = left + 1;

    let left_cost = pos_cost_both_sides(left, left_crab, right_crab);
    let right_cost = pos_cost_both_sides(right, left_crab, right_crab);
    if left_cost < right_cost {
        (left_cost, left_crab.pos + left)
    } else {
        (right_cost, left_crab.pos + right)
    }
}

fn step_2(start_positions: &[u16]) {
    let mut crab_positions: Vec<_> = start_positions.iter().cloned().map(i64::from).collect();
    crab_positions.sort();

    let moving_costs_from_left = crab_moving_costs_one_direction(crab_positions.iter().cloned());
    let moving_costs_from_right =
        crab_moving_costs_one_direction(crab_positions.iter().cloned().rev());

    struct CrabMovingCost {
        from_the_left: CrabMoveCost,
        from_the_right: CrabMoveCost,
    }
    let total: Vec<_> = moving_costs_from_left
        .into_iter()
        .zip(moving_costs_from_right.into_iter().rev())
        .map(|(l, r)| CrabMovingCost {
            from_the_left: l,
            from_the_right: r,
        })
        .collect();

    let min_cost = total
        .array_windows::<2>()
        .map(|[l, r]| crab_segment_min_cost(&l.from_the_left, &r.from_the_right))
        .min()
        .unwrap();
    println!(
        "Second step result: min cost {} attained for {}",
        min_cost.0, min_cost.1
    );
}

fn main() {
    let input = parse_input();

    step_1(&input);
    step_2(&input);
}
