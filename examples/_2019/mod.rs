#![allow(unused)]

use anyhow::{anyhow, Result};

#[derive(Debug, Copy, Clone)]
pub enum IntCode {
    Position(usize),
    Immediate(isize),
}

impl IntCode {
    pub fn as_value(&self, memory: &[isize]) -> isize {
        match self {
            IntCode::Immediate(x) => *x,
            IntCode::Position(i) => memory[*i],
        }
    }

    pub fn as_position(&self) -> Option<&usize> {
        match self {
            IntCode::Immediate(_) => None,
            IntCode::Position(i) => Some(i),
        }
    }
}

#[derive(Debug)]
pub enum OpCode {
    Add {
        lhs: IntCode,
        rhs: IntCode,
        target: IntCode,
    },
    Mult {
        lhs: IntCode,
        rhs: IntCode,
        target: IntCode,
    },
    Input {
        target: IntCode,
    },
    Output {
        target: IntCode,
    },
    JumpIfTrue {
        test: IntCode,
        value: IntCode,
    },
    JumpIfFalse {
        test: IntCode,
        value: IntCode,
    },
    LessThan {
        lhs: IntCode,
        rhs: IntCode,
        target: IntCode,
    },
    Equals {
        lhs: IntCode,
        rhs: IntCode,
        target: IntCode,
    },
    Halt,
}

impl OpCode {
    pub fn read(ip: &mut usize, slice: &[isize]) -> Result<OpCode> {
        let start_ip = *ip;
        let instruction = slice[*ip];
        *ip += 1;

        // get the right int code for the parameter mode
        let mut next_int_code = || {
            let int_code = slice
                .get(*ip)
                .map(|x| {
                    if aoc_lib::utils::digit_at(instruction as usize, (*ip - start_ip) + 1) == 1 {
                        IntCode::Immediate(*x)
                    } else {
                        IntCode::Position(*x as usize)
                    }
                })
                .ok_or_else(|| anyhow!("No value at position: {}", ip));

            *ip += 1;
            int_code
        };

        Ok(match instruction % 100 {
            1 => OpCode::Add {
                lhs: next_int_code()?,
                rhs: next_int_code()?,
                target: next_int_code()?,
            },
            2 => OpCode::Mult {
                lhs: next_int_code()?,
                rhs: next_int_code()?,
                target: next_int_code()?,
            },
            3 => OpCode::Input {
                target: next_int_code()?,
            },
            4 => OpCode::Output {
                target: next_int_code()?,
            },
            5 => OpCode::JumpIfTrue {
                test: next_int_code()?,
                value: next_int_code()?,
            },
            6 => OpCode::JumpIfFalse {
                test: next_int_code()?,
                value: next_int_code()?,
            },
            7 => OpCode::LessThan {
                lhs: next_int_code()?,
                rhs: next_int_code()?,
                target: next_int_code()?,
            },
            8 => OpCode::Equals {
                lhs: next_int_code()?,
                rhs: next_int_code()?,
                target: next_int_code()?,
            },
            99 => OpCode::Halt,
            _ => panic!(
                "Received unrecognised instruction: {} at index: {}",
                instruction, *ip
            ),
        })
    }
}

pub struct Program<'a> {
    ip: usize,
    memory: Vec<isize>,

    handle_input: Option<&'a dyn Fn() -> isize>,
    handle_output: Option<&'a mut dyn FnMut(isize) -> bool>,
}

impl<'a> Program<'a> {
    pub fn new(int_codes: Vec<isize>) -> Program<'a> {
        Program {
            ip: 0,
            memory: int_codes,
            handle_input: None,
            handle_output: None,
        }
    }

    pub fn get_ip(&self) -> usize {
        self.ip
    }

    pub fn get_memory(&self) -> Vec<isize> {
        self.memory.clone()
    }

    pub fn set_memory(&mut self, memory: Vec<isize>) {
        self.memory = memory;
    }

    pub fn set_handle_input(&mut self, f: &'a dyn Fn() -> isize) {
        self.handle_input = Some(f);
    }

    pub fn set_handle_output(&mut self, f: &'a mut dyn FnMut(isize) -> bool) {
        self.handle_output = Some(f);
    }

    pub fn run(&mut self) {
        while let Ok(op_code) = OpCode::read(&mut self.ip, &self.memory) {
            match op_code {
                OpCode::Add { lhs, rhs, target } => {
                    self.memory[*target.as_position().unwrap()] =
                        lhs.as_value(&self.memory) + rhs.as_value(&self.memory);
                }
                OpCode::Mult { lhs, rhs, target } => {
                    self.memory[*target.as_position().unwrap()] =
                        lhs.as_value(&self.memory) * rhs.as_value(&self.memory);
                }
                OpCode::Input { target } => {
                    self.memory[*target.as_position().unwrap()] =
                        (self.handle_input.as_ref().unwrap())()
                }
                OpCode::Output { target } => {
                    if (self.handle_output.as_mut().unwrap())(target.as_value(&self.memory)) {
                        break;
                    }
                }
                OpCode::JumpIfTrue { test, value } => {
                    if test.as_value(&self.memory) != 0 {
                        self.ip = value.as_value(&self.memory) as usize;
                    }
                }
                OpCode::JumpIfFalse { test, value } => {
                    if test.as_value(&self.memory) == 0 {
                        self.ip = value.as_value(&self.memory) as usize;
                    }
                }
                OpCode::LessThan { lhs, rhs, target } => {
                    self.memory[*target.as_position().unwrap()] =
                        if lhs.as_value(&self.memory) < rhs.as_value(&self.memory) {
                            1
                        } else {
                            0
                        }
                }
                OpCode::Equals { lhs, rhs, target } => {
                    self.memory[*target.as_position().unwrap()] =
                        if lhs.as_value(&self.memory) == rhs.as_value(&self.memory) {
                            1
                        } else {
                            0
                        }
                }
                OpCode::Halt => break,
            }
        }
    }
}
