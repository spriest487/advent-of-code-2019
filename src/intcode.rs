use std::convert::TryInto;
use digits_iterator;
use digits_iterator::DigitsExtension;

pub type Word = i64;

fn as_addr(word: Word) -> usize {
    match word.try_into() {
        Ok(addr) => addr,
        Err(..) => panic!("bad address: {}", word),
    }
}

fn load(code: &[Word], mode: Mode, val: Word) -> Word {
    match mode {
        Mode::Pointer => code[val as usize],
        Mode::Immediate => val,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Op {
    Add,
    Mul,
    In,
    Out,
    Hcf,
    Jz,
    Jnz,
    Lt,
    Eq,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Mode {
    Pointer,
    Immediate,
}

struct OpCode {
    op: Op,
    param_modes: Vec<Mode>,
}

impl From<Word> for OpCode {
    fn from(word: Word) -> Self {
        let mut digits = word.digits().rev();
        let op = {
            let lo = digits.next().unwrap();

            let code = if let Some(hi) = digits.next() {
                lo + hi * 10
            } else {
                lo
            };

            match code {
                1 => Op::Add,
                2 => Op::Mul,
                3 => Op::In,
                4 => Op::Out,
                5 => Op::Jnz,
                6 => Op::Jz,
                7 => Op::Lt,
                8 => Op::Eq,
                99 => Op::Hcf,
                _ => panic!("bad op value in opcode {}: {}", word, code),
            }
        };

        let mut param_modes = Vec::new();
        while let Some(mode) = digits.next() {
            param_modes.push(match mode {
                0 => Mode::Pointer,
                1 => Mode::Immediate,
                _ => panic!("bad mode digit in opcode {}: {}", word, mode)
            })
        }

        Self {
            op,
            param_modes,
        }
    }
}

impl OpCode {
    fn param_mode(&self, param: usize) -> Mode {
        self.param_modes.get(param).cloned().unwrap_or(Mode::Pointer)
    }
}

pub fn exec(intcode: &mut [Word], in_buf: &mut Vec<Word>, out_buf: &mut Vec<Word>) {
    let mut pc = 0;
    loop {
        let opcode = OpCode::from(intcode[pc]);

        match opcode.op {
            Op::Add => {
                let a = load(intcode, opcode.param_mode(0), intcode[pc + 1]);
                let b = load(intcode, opcode.param_mode(1), intcode[pc + 2]);
                assert_eq!(opcode.param_mode(2), Mode::Pointer);
                let out_pos = as_addr(intcode[pc + 3]);

                intcode[out_pos] = a + b;
                pc += 4;
            }

            Op::Mul => {
                let a = load(intcode, opcode.param_mode(0), intcode[pc + 1]);
                let b = load(intcode, opcode.param_mode(1), intcode[pc + 2]);
                assert_eq!(opcode.param_mode(2), Mode::Pointer);
                let out_pos = as_addr(intcode[pc + 3]);

                intcode[out_pos] = a * b;
                pc += 4;
            }

            Op::Jnz => {
                let a = load(intcode, opcode.param_mode(0), intcode[pc + 1]);
                let b = load(intcode, opcode.param_mode(1), intcode[pc + 2]);

                if a != 0 {
                    pc = as_addr(b);
                } else {
                    pc += 3;
                }
            }

            Op::Jz => {
                let a = load(intcode, opcode.param_mode(0), intcode[pc + 1]);
                let b = load(intcode, opcode.param_mode(1), intcode[pc + 2]);

                if a == 0 {
                    pc = as_addr(b);
                } else {
                    pc += 3;
                }
            }

            Op::In => {
                assert_eq!(opcode.param_mode(0), Mode::Pointer);
                let pos = as_addr(intcode[pc + 1]);

                let in_val = in_buf.remove(0);
                intcode[pos] = in_val;

                pc += 2;
            }

            Op::Out => {
                let out_val = load(&intcode, opcode.param_mode(0), intcode[pc + 1]);
                out_buf.push(out_val);
                pc += 2;
            }

            Op::Lt => {
                let a = load(intcode, opcode.param_mode(0), intcode[pc + 1]);
                let b = load(intcode, opcode.param_mode(1), intcode[pc + 2]);

                assert_eq!(opcode.param_mode(2), Mode::Pointer);
                let out_pos = as_addr(intcode[pc + 3]);

                intcode[out_pos] = if a < b { 1 } else { 0 };
                pc += 4;
            }

            Op::Eq => {
                let a = load(intcode, opcode.param_mode(0), intcode[pc + 1]);
                let b = load(intcode, opcode.param_mode(1), intcode[pc + 2]);

                assert_eq!(opcode.param_mode(2), Mode::Pointer);
                let out_pos = as_addr(intcode[pc + 3]);

                intcode[out_pos] = if a == b { 1 } else { 0 };
                pc += 4;
            }

            Op::Hcf => break,
        }
    }
}

pub fn from_str(input: &str) -> Vec<Word> {
    input.split(",").map(|int| int.parse().unwrap()).collect()
}