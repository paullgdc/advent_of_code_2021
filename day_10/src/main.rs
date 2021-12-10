use std::{collections::HashMap, io::BufRead};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum BracketType {
    Straight,
    Parenthesis,
    Curly,
    Angle,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Orientation {
    Open,
    Close,
}

fn parse_input() -> Vec<Vec<(BracketType, Orientation)>> {
    use BracketType::*;
    use Orientation::*;
    std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    '(' => (Parenthesis, Open),
                    ')' => (Parenthesis, Close),
                    '[' => (Straight, Open),
                    ']' => (Straight, Close),
                    '<' => (Angle, Open),
                    '>' => (Angle, Close),
                    '{' => (Curly, Open),
                    '}' => (Curly, Close),
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn first_illegal_char(
    line: &[(BracketType, Orientation)],
) -> Result<Vec<BracketType>, BracketType> {
    let mut stack = Vec::new();
    for (bracket, orientation) in line {
        match orientation {
            Orientation::Open => stack.push(*bracket),
            Orientation::Close => {
                let matching = stack.pop();
                if matching != Some(*bracket) {
                    return Err(*bracket);
                }
            }
        }
    }
    Ok(stack)
}

fn step_1(lines: &[Vec<(BracketType, Orientation)>]) {
    let mut illegals = HashMap::new();
    for line in lines {
        match first_illegal_char(&*line) {
            Ok(_) => continue,
            Err(b) => {
                *illegals.entry(b).or_insert(0) += 1;
            }
        }
    }
    let result: u64 = illegals
        .iter()
        .map(|(bracket, count)| {
            count
                * match bracket {
                    BracketType::Parenthesis => 3,
                    BracketType::Straight => 57,
                    BracketType::Curly => 1197,
                    BracketType::Angle => 25137,
                }
        })
        .sum();
    println!("First step result: {}", result);
}

fn step_2(lines: &[Vec<(BracketType, Orientation)>]) {
    let mut lines_scores = Vec::new();
    for line in lines {
        match first_illegal_char(&*line) {
            Ok(remaining_stack) if !remaining_stack.is_empty() => {
                let mut score: u64 = 0;
                for left in remaining_stack.iter().rev() {
                    score *= 5;
                    score += match left {
                        BracketType::Parenthesis => 1,
                        BracketType::Straight => 2,
                        BracketType::Curly => 3,
                        BracketType::Angle => 4,
                    };
                }
                lines_scores.push(score);
            }
            Ok(_) | Err(_) => continue,
        }
    }
    lines_scores.sort();
    assert!(lines_scores.len() % 2 == 1);
    let result = lines_scores[lines_scores.len() / 2];
    println!("First step result: {}", result);
}

fn main() {
    let lines = parse_input();
    step_1(&lines);
    step_2(&lines);
}
