use super::op_code::OpCode;
use super::traits::{IntRead, IntWrite};
use super::Int;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StopReason {
    Halt,
    WaitingForInput,
}

#[derive(Clone)]
pub struct Program {
    ip: usize,
    memory: Vec<Int>,
}

impl Program {
    pub fn new(ints: Vec<Int>) -> Program {
        Program {
            ip: 0,
            memory: ints,
        }
    }

    pub fn get_memory(&self) -> Vec<Int> {
        self.memory.clone()
    }

    pub fn run_no_io(&mut self) -> StopReason {
        self.run(&mut vec![], &mut vec![])
    }

    pub fn run<R, W>(&mut self, mut input: R, mut output: W) -> StopReason
    where
        R: IntRead,
        W: IntWrite,
    {
        while let Ok(op_code) = OpCode::next(&mut self.ip, &self.memory) {
            match op_code {
                OpCode::Add { lhs, rhs, target } => {
                    self.memory[*target.as_position().unwrap()] =
                        lhs.as_int(&self.memory) + rhs.as_int(&self.memory);
                }
                OpCode::Mult { lhs, rhs, target } => {
                    self.memory[*target.as_position().unwrap()] =
                        lhs.as_int(&self.memory) * rhs.as_int(&self.memory);
                }
                OpCode::Input { target } => match input.int_read() {
                    Some(x) => self.memory[*target.as_position().unwrap()] = x,
                    None => {
                        // rewind the instruction pointer to the Input instruction
                        self.ip -= op_code.len();
                        return StopReason::WaitingForInput;
                    }
                },
                OpCode::Output { target } => output.int_write(target.as_int(&self.memory)),
                OpCode::JumpIfTrue { test, destination } => {
                    if test.as_int(&self.memory) != 0 {
                        self.ip = destination.as_int(&self.memory) as usize;
                    }
                }
                OpCode::JumpIfFalse { test, destination } => {
                    if test.as_int(&self.memory) == 0 {
                        self.ip = destination.as_int(&self.memory) as usize;
                    }
                }
                OpCode::LessThan { lhs, rhs, target } => {
                    self.memory[*target.as_position().unwrap()] =
                        if lhs.as_int(&self.memory) < rhs.as_int(&self.memory) {
                            1
                        } else {
                            0
                        }
                }
                OpCode::Equals { lhs, rhs, target } => {
                    self.memory[*target.as_position().unwrap()] =
                        if lhs.as_int(&self.memory) == rhs.as_int(&self.memory) {
                            1
                        } else {
                            0
                        }
                }
                OpCode::Halt => return StopReason::Halt,
            }
        }

        unreachable!("Unexpected end of program!")
    }
}
