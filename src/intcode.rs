#![allow(unused)]

use std::convert::TryInto;
use digits_iterator;
use digits_iterator::DigitsExtension;

pub type Word = i64;

#[derive(Debug)]
pub enum ExecError {
    InputBlocked,
}

pub type ExecResult<T> = Result<T, ExecError>;

fn as_addr(word: Word) -> usize {
    match word.try_into() {
        Ok(addr) => addr,
        Err(..) => panic!("bad address: {}", word),
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
    Off,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Mode {
    Pointer,
    Immediate,
    Relative,
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
                9 => Op::Off,
                99 => Op::Hcf,
                _ => panic!("bad op value in opcode {}: {}", word, code),
            }
        };

        let mut param_modes = Vec::new();
        while let Some(mode) = digits.next() {
            param_modes.push(match mode {
                0 => Mode::Pointer,
                1 => Mode::Immediate,
                2 => Mode::Relative,
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

pub fn from_str(input: &str) -> Vec<Word> {
    input.split(",").map(|int| int.parse().unwrap()).collect()
}

pub struct Computer {
    pub in_buf: Vec<Word>,
    pub out_buf: Vec<Word>,

    mem: Vec<Word>,

    pc: usize,
    rel_offset: Word,
}

impl Computer {
    pub fn new(code: Vec<Word>) -> Self {
        Self {
            in_buf: Vec::new(),
            out_buf: Vec::new(),

            mem: code,

            pc: 0,
            rel_offset: 0,
        }
    }

    fn load(&mut self, mode: Mode, val: Word) -> Word {
        match mode {
            Mode::Pointer => self.mem_load(as_addr(val)),
            Mode::Immediate => val,
            Mode::Relative => self.mem_load(as_addr(val + self.rel_offset)),
        }
    }

    fn mem_load(&mut self, addr: usize) -> Word {
        self.mem.get(addr).cloned().unwrap_or(0)
    }

    fn mem_store(&mut self, addr: usize, val: Word) {
        // todo: pages
        while self.mem.len() < addr {
            self.mem.resize(self.mem.len() * 2, 0);
        }

        self.mem[addr] = val;
    }

    pub fn code_get(&self, at: usize) -> Option<Word> {
        self.mem.get(at).cloned()
    }

    fn get_ptr(&self, mode: Mode, val: Word) -> usize {
        as_addr(match mode {
            Mode::Pointer => val,
            Mode::Relative => val + self.rel_offset,
            Mode::Immediate => panic!("in instruction doesn't allow immediate arg"),
        })
    }

    pub fn run(&mut self) -> ExecResult<()> {
        loop {
            let opcode = OpCode::from(self.mem[self.pc]);

            match opcode.op {
                Op::Add => {
                    let a_pos = self.mem_load(self.pc + 1);
                    let a = self.load(opcode.param_mode(0), a_pos);

                    let b_pos = self.mem_load(self.pc + 2);
                    let b = self.load(opcode.param_mode(1), b_pos);

                    let out = self.mem_load(self.pc + 3);
                    let out_pos = self.get_ptr(opcode.param_mode(2), out);

                    self.mem_store(out_pos, a + b);
                    self.pc += 4;
                }

                Op::Mul => {
                    let a_pos = self.mem_load(self.pc + 1);
                    let a = self.load(opcode.param_mode(0), a_pos);

                    let b_pos = self.mem_load(self.pc + 2);
                    let b = self.load(opcode.param_mode(1), b_pos);

                    let out = self.mem_load(self.pc + 3);
                    let out_pos = self.get_ptr(opcode.param_mode(2), out);

                    self.mem_store(out_pos, a * b);
                    self.pc += 4;
                }

                Op::Jnz => {
                    let a_pos = self.mem_load(self.pc + 1);
                    let a = self.load(opcode.param_mode(0), a_pos);

                    let b_pos = self.mem_load(self.pc + 2);
                    let b = self.load(opcode.param_mode(1), b_pos);

                    if a != 0 {
                        self.pc = as_addr(b);
                    } else {
                        self.pc += 3;
                    }
                }

                Op::Jz => {
                    let a_pos = self.mem_load(self.pc + 1);
                    let a = self.load(opcode.param_mode(0), a_pos);

                    let b_pos = self.mem_load(self.pc + 2);
                    let b = self.load(opcode.param_mode(1), b_pos);

                    if a == 0 {
                        self.pc = as_addr(b);
                    } else {
                        self.pc += 3;
                    }
                }

                Op::In => {
                    if self.in_buf.is_empty() {
                        return Err(ExecError::InputBlocked);
                    }

                    let at = self.mem_load(self.pc + 1);
                    let at_pos = self.get_ptr(opcode.param_mode(0), at);

                    let in_val = self.in_buf.remove(0);
                    self.mem_store(at_pos, in_val);

                    self.pc += 2;
                }

                Op::Out => {
                    let val_pos = self.mem_load(self.pc + 1);
                    let val = self.load(opcode.param_mode(0), val_pos);

                    self.out_buf.push(val);
                    self.pc += 2;
                }

                Op::Lt => {
                    let a_pos = self.mem_load(self.pc + 1);
                    let a = self.load(opcode.param_mode(0), a_pos);

                    let b_pos = self.mem_load(self.pc + 2);
                    let b = self.load(opcode.param_mode(1), b_pos);

                    let out = self.mem_load(self.pc + 3);
                    let out_pos = self.get_ptr(opcode.param_mode(2), out);

                    self.mem_store(out_pos, if a < b { 1 } else { 0 });
                    self.pc += 4;
                }

                Op::Eq => {
                    let a_pos = self.mem_load(self.pc + 1);
                    let a = self.load(opcode.param_mode(0), a_pos);

                    let b_pos = self.mem_load(self.pc + 2);
                    let b = self.load(opcode.param_mode(1), b_pos);

                    let out = self.mem_load(self.pc + 3);
                    let out_pos = self.get_ptr(opcode.param_mode(2), out);

                    self.mem_store(out_pos, if a == b { 1 } else { 0 });
                    self.pc += 4;
                }

                Op::Off => {
                    let a_pos = self.mem_load(self.pc + 1);
                    let a = self.load(opcode.param_mode(0), a_pos);

                    self.rel_offset += a;
                    self.pc += 2;
                }

                Op::Hcf => break Ok(()),
            }
        }
    }
}