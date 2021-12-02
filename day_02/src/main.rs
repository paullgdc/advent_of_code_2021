use std::{io::BufRead, panic};

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_input() -> Vec<Command> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let (command, length) = line.trim().split_once(" ").unwrap();
            let length = length.parse().unwrap();
            match command {
                "forward" => Command::Forward(length),
                "up" => Command::Up(length),
                "down" => Command::Down(length),
                _ => panic!("Unrecogmized command"),
            }
        })
        .collect()
}

fn apply_commands(commands: &[Command]) -> i32 {
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;

    for command in commands {
        match command {
            Command::Forward(length) => horizontal_pos += length,
            Command::Up(length) => vertical_pos -= length,
            Command::Down(length) => vertical_pos += length,
        }
    }

    horizontal_pos * vertical_pos
}

fn apply_commands_with_aim(commands: &[Command]) -> i32 {
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Forward(length) => {
                horizontal_pos += length;
                vertical_pos += length * aim
            }
            Command::Up(length) => aim -= length,
            Command::Down(length) => aim += length,
        }
    }

    horizontal_pos * vertical_pos
}

fn main() {
    let commands = parse_input();

    let first_step_sol = apply_commands(&commands);
    println!("First step solution: {}", first_step_sol);

    let second_step_sol = apply_commands_with_aim(&commands);
    println!("Second step solution: {}", second_step_sol);
}
