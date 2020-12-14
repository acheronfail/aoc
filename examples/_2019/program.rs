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
    /// Instruction Pointer
    ip: usize,
    /// Relative Base (see aoc 2019-9)
    rb: usize,
    /// Program's memory (the raw int code)
    memory: Vec<Int>,
}

impl Program {
    pub fn new(mut ints: Vec<Int>) -> Program {
        // reserve more space in the program's memory
        ints.resize(ints.len() * 2, 0);

        Program {
            ip: 0,
            rb: 0,
            memory: ints,
        }
    }

    pub fn get_memory(&self) -> Vec<Int> {
        self.memory.clone()
    }

    pub fn run_no_io(&mut self) -> StopReason {
        self.run(None, &mut vec![])
    }

    pub fn run<R, W>(&mut self, mut input: R, mut output: W) -> StopReason
    where
        R: IntRead,
        W: IntWrite,
    {
        while let Ok(op_code) = OpCode::next(&mut self.ip, &self.memory) {
            match op_code {
                OpCode::Add { lhs, rhs, target } => {
                    self.memory[target.as_address(self.rb).unwrap()] =
                        lhs.as_int(&self.memory, self.rb) + rhs.as_int(&self.memory, self.rb);
                }
                OpCode::Mult { lhs, rhs, target } => {
                    self.memory[target.as_address(self.rb).unwrap()] =
                        lhs.as_int(&self.memory, self.rb) * rhs.as_int(&self.memory, self.rb);
                }
                OpCode::Input { target } => match input.int_read() {
                    Some(x) => self.memory[target.as_address(self.rb).unwrap()] = x,
                    None => {
                        // rewind the instruction pointer to the Input instruction
                        self.ip -= op_code.len();
                        return StopReason::WaitingForInput;
                    }
                },
                OpCode::Output { target } => output.int_write(target.as_int(&self.memory, self.rb)),
                OpCode::JumpIfTrue { test, destination } => {
                    if test.as_int(&self.memory, self.rb) != 0 {
                        self.ip = destination.as_int(&self.memory, self.rb) as usize;
                    }
                }
                OpCode::JumpIfFalse { test, destination } => {
                    if test.as_int(&self.memory, self.rb) == 0 {
                        self.ip = destination.as_int(&self.memory, self.rb) as usize;
                    }
                }
                OpCode::LessThan { lhs, rhs, target } => {
                    self.memory[target.as_address(self.rb).unwrap()] =
                        if lhs.as_int(&self.memory, self.rb) < rhs.as_int(&self.memory, self.rb) {
                            1
                        } else {
                            0
                        }
                }
                OpCode::Equals { lhs, rhs, target } => {
                    self.memory[target.as_address(self.rb).unwrap()] =
                        if lhs.as_int(&self.memory, self.rb) == rhs.as_int(&self.memory, self.rb) {
                            1
                        } else {
                            0
                        }
                }
                OpCode::AdjustRelativeBase { amount } => {
                    self.rb = (self.rb as Int + amount.as_int(&self.memory, self.rb)) as usize;
                }
                OpCode::Halt => return StopReason::Halt,
            }
        }

        unreachable!("Unexpected end of program!")
    }
}
