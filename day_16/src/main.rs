#![feature(let_else)]
use std::io::BufRead;

struct BitArray<'a> {
    array: &'a [u8],
    bit_idx: usize,
}

impl<'a> BitArray<'a> {
    fn next_byte(&mut self) -> Option<u8> {
        let b = (self.array.get(self.bit_idx / 8)? << (self.bit_idx % 8)) >> 7;
        self.bit_idx += 1;
        Some(b as u8)
    }

    fn next_n_bytes(&mut self, n: usize) -> Option<u16> {
        let mut r = 0;
        for _ in 0..n {
            r <<= 1;
            r |= self.next_byte()? as u16;
        }
        Some(r)
    }
}

#[test]
fn test_bit_array() {
    let mut m = BitArray {
        array: &vec![0b10110001, 0b1111],
        bit_idx: 0,
    };
    assert_eq!(m.next_byte().unwrap(), 1);
    assert_eq!(m.next_byte().unwrap(), 0);
    assert_eq!(m.next_byte().unwrap(), 1);
    assert_eq!(m.next_byte().unwrap(), 1);
    assert_eq!(m.next_byte().unwrap(), 0);
    assert_eq!(m.next_byte().unwrap(), 0);
    assert_eq!(m.next_byte().unwrap(), 0);
    assert_eq!(m.next_byte().unwrap(), 1);

    for _ in 0..3 {
        assert_eq!(m.next_byte().unwrap(), 0);
    }
    assert_eq!(m.next_n_bytes(3), Some(0b011));
    for _ in 0..2 {
        assert_eq!(m.next_byte().unwrap(), 1);
    }
    assert_eq!(m.next_byte(), None);
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

#[derive(Debug)]
enum Message {
    Litteral(u64),
    Ops { operands: u8, op: Op },
}

trait Listener {
    fn enter(&mut self, version: u8) {
        let _ = version;
    }
    fn exit(&mut self, message: Message) {
        let _ = message;
    }
}

fn parse_input() -> Vec<u8> {
    let message = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let mut parts = message
        .trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8);
    let mut array = Vec::new();
    loop {
        let Some(n) = parts.next() else {
            break;
        };
        array.push(n << 4 | parts.next().unwrap_or(0))
    }
    array
}

fn parse_message<L: Listener>(bits: &mut BitArray, listener: &mut L) -> Option<()> {
    let version = bits.next_n_bytes(3)? as u8;
    let type_id = bits.next_n_bytes(3)? as u8;
    listener.enter(version);
    if type_id == 4 {
        parse_litteral(bits, listener)?;
    } else {
        parse_operator(bits, type_id, listener)?;
    }
    Some(())
}

fn parse_litteral<L: Listener>(bits: &mut BitArray, listener: &mut L) -> Option<()> {
    let mut value = 0;
    while bits.next_byte()? == 1 {
        value |= bits.next_n_bytes(4)? as u64;
        value <<= 4;
    }
    value |= bits.next_n_bytes(4)? as u64;
    listener.exit(Message::Litteral(value));
    Some(())
}

fn parse_operator<L: Listener>(bits: &mut BitArray, type_id: u8, listener: &mut L) -> Option<()> {
    let length_type_id = bits.next_byte().unwrap();
    let operands = match length_type_id {
        1 => {
            let operands = bits.next_n_bytes(11).unwrap();
            for _ in 0..operands {
                parse_message(bits, listener)?;
            }
            operands
        }
        0 => {
            let sub_length = bits.next_n_bytes(15).unwrap();
            let end = bits.bit_idx + sub_length as usize;
            let mut operands = 0;
            while bits.bit_idx < end {
                parse_message(bits, listener)?;
                operands += 1;
            }
            operands
        }
        _ => panic!(),
    } as u8;
    listener.exit(Message::Ops {
        operands,
        op: match type_id {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Min,
            3 => Op::Max,
            5 => Op::Gt,
            6 => Op::Lt,
            7 => Op::Eq,
            _ => panic!(),
        },
    });
    Some(())
}

struct VersionSumListener {
    sum: u64,
}

impl Listener for VersionSumListener {
    fn enter(&mut self, version: u8) {
        self.sum += version as u64;
    }
}

struct StackInterpreterListener {
    stack: Vec<u64>,
}

impl Listener for StackInterpreterListener {
    fn exit(&mut self, message: Message) {
        let (op, operands) = match message {
            Message::Litteral(v) => {
                self.stack.push(v);
                return;
            }
            Message::Ops { operands, op } => {
                (op, &self.stack[(self.stack.len() - (operands as usize))..])
            }
        };
        let res: u64 = match op {
            Op::Sum => operands.iter().sum(),
            Op::Product => operands.iter().product(),
            Op::Min => *operands.iter().min().unwrap(),
            Op::Max => *operands.iter().max().unwrap(),
            Op::Gt => {
                if operands[0] > operands[1] {
                    1
                } else {
                    0
                }
            }
            Op::Lt => {
                if operands[0] < operands[1] {
                    1
                } else {
                    0
                }
            }
            Op::Eq => {
                if operands[0] == operands[1] {
                    1
                } else {
                    0
                }
            }
        };
        let operands_len = operands.len();
        self.stack.truncate(self.stack.len() - operands_len);
        self.stack.push(res);
    }
}

fn step_1(input: &[u8]) {
    let mut message = BitArray {
        array: input.into(),
        bit_idx: 0,
    };
    let mut sum = VersionSumListener { sum: 0 };
    parse_message(&mut message, &mut sum);
    println!("First step result: {}", sum.sum);
}

fn step_2(input: &[u8]) {
    let mut message = BitArray {
        array: input.into(),
        bit_idx: 0,
    };
    let mut interpeter = StackInterpreterListener { stack: Vec::new() };
    parse_message(&mut message, &mut interpeter);
    println!("Second step result: {}", interpeter.stack[0]);
}

fn main() {
    let input = parse_input();
    step_1(&input);
    step_2(&input);
}
