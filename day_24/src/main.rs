#![feature(let_else)]

use std::io::Read;

type Val = i32;

#[derive(Debug)]
struct RegisterState([Val; 4]);

impl RegisterState {
    fn new() -> Self {
        Self([0; 4])
    }
    fn get_op(&self, op: &Op<Reg>) -> Val {
        match op {
            Op::Const(c) => *c,
            Op::Register(reg) => self.get_reg(reg),
        }
    }

    fn get_reg(&self, reg: &Reg) -> Val {
        self.0[reg.to_pos()]
    }

    fn set(&mut self, reg: &Reg, val: Val) {
        self.0[reg.to_pos()] = val
    }
}

#[derive(Clone, Copy)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

struct UniqueReg {
    id: i32,
    reg: Reg,
}

enum Op<R> {
    Register(R),
    Const(i32),
}

impl Reg {
    fn to_pos(&self) -> usize {
        match self {
            Reg::W => 0,
            Reg::X => 1,
            Reg::Y => 2,
            Reg::Z => 3,
        }
    }
}

enum Instruction<U, V> {
    Inp(U),
    Add((U, V)),
    Mul((U, V)),
    Div((U, V)),
    Mod((U, V)),
    Eql((U, V)),
}

fn parse_input(input: &str) -> Vec<Instruction<Reg, Op<Reg>>> {
    fn parse_reg(reg: &str) -> Option<Reg> {
        Some(match reg {
            "w" => Reg::W,
            "x" => Reg::X,
            "y" => Reg::Y,
            "z" => Reg::Z,
            _ => return None,
        })
    }
    fn parse_op(op: &str) -> Op<Reg> {
        match op {
            "w" | "x" | "y" | "z" => Op::Register(parse_reg(op).unwrap()),
            val => Op::Const(val.parse().unwrap()),
        }
    }

    fn parse_two_reg(input: &str) -> (Reg, Op<Reg>) {
        let (f, s) = input.split_once(" ").unwrap();
        (parse_reg(f).unwrap(), parse_op(s))
    }

    input
        .lines()
        .map(|l| {
            let (instr, regs) = l.split_once(" ").unwrap();
            match instr {
                "inp" => Instruction::Inp(parse_reg(regs).unwrap()),
                "add" => Instruction::Add(parse_two_reg(regs)),
                "mul" => Instruction::Mul(parse_two_reg(regs)),
                "div" => Instruction::Div(parse_two_reg(regs)),
                "mod" => Instruction::Mod(parse_two_reg(regs)),
                "eql" => Instruction::Eql(parse_two_reg(regs)),
                _ => panic!(),
            }
        })
        .collect()
}

fn eval_instruction(
    instr: &Instruction<Reg, Op<Reg>>,
    mut inputs: impl Iterator<Item = Val>,
    state: &mut RegisterState,
) -> Option<()> {
    match instr {
        Instruction::Add((r1, r2)) => state.set(r1, state.get_reg(r1) + state.get_op(r2)),
        Instruction::Div((r1, r2)) => state.set(r1, state.get_reg(r1) / state.get_op(r2)),
        Instruction::Eql((r1, r2)) => state.set(
            r1,
            if state.get_reg(r1) == state.get_op(r2) {
                1
            } else {
                0
            },
        ),
        Instruction::Inp(r1) => state.set(r1, inputs.next().unwrap()),
        Instruction::Mod((r1, r2)) => state.set(r1, state.get_reg(r1) % state.get_op(r2)),
        Instruction::Mul((r1, r2)) => state.set(r1, state.get_reg(r1) * state.get_op(r2)),
    }
    Some(())
}

fn eval_instructions(instrs: &[Instruction<Reg, Op<Reg>>], inputs: &[i32]) -> RegisterState {
    let mut state = RegisterState([0; 4]);
    let mut inputs = inputs.into_iter().copied();
    for instr in instrs {
        eval_instruction(instr, &mut inputs, &mut state);
    }
    state
}

fn instr_to_unique(
    instrs: &[Instruction<Reg, Op<Reg>>],
) -> Vec<Instruction<UniqueReg, Op<UniqueReg>>> {
    fn map_reg(reg_state: &mut RegisterState, reg: &Reg, incr: bool) -> UniqueReg {
        let mut id = reg_state.get_reg(reg);
        if incr {
            id += 1;
            reg_state.set(reg, id)
        }
        UniqueReg { id, reg: *reg }
    }
    fn map_op(reg_state: &mut RegisterState, op: &Op<Reg>) -> Op<UniqueReg> {
        match op {
            Op::Const(c) => Op::Const(*c),
            Op::Register(reg) => Op::Register(map_reg(reg_state, reg, false)),
        }
    }
    let mut reg_state = RegisterState::new();
    instrs
        .into_iter()
        .map(|i| {
            use Instruction::*;
            match i {
                Inp(r) => Inp(map_reg(&mut reg_state, r, true)),
                Add((r, o)) => {
                    let operand = map_op(&mut reg_state, o);
                    let receiver = map_reg(&mut reg_state, r, true);
                    Add((receiver, operand))
                }
                Mul((r, o)) => {
                    let operand = map_op(&mut reg_state, o);
                    let receiver = map_reg(&mut reg_state, r, true);
                    Mul((receiver, operand))
                }
                Div((r, o)) => {
                    let operand = map_op(&mut reg_state, o);
                    let receiver = map_reg(&mut reg_state, r, true);
                    Div((receiver, operand))
                }
                Mod((r, o)) => {
                    let operand = map_op(&mut reg_state, o);
                    let receiver = map_reg(&mut reg_state, r, true);
                    Mod((receiver, operand))
                }
                Eql((r, o)) => {
                    let operand = map_op(&mut reg_state, o);
                    let receiver = map_reg(&mut reg_state, r, true);
                    Eql((receiver, operand))
                }
            }
        })
        .collect()
}

fn main() {
    let mut input = Vec::new();
    std::io::stdin().lock().read_to_end(&mut input).unwrap();
    let input = std::str::from_utf8(&input).unwrap();

    let instructions = parse_input(input);
    for i in 0..16 {
        dbg!(eval_instructions(&instructions, &[i]));
    }
}
