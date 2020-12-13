use super::int_code::IntCode;
use super::Int;
use anyhow::{anyhow, Result};

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
        destination: IntCode,
    },
    JumpIfFalse {
        test: IntCode,
        destination: IntCode,
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
    /// How many `Int`s this `OpCode` is built from
    pub fn len(&self) -> usize {
        match self {
            OpCode::Halt => 1,
            OpCode::Input { .. } | OpCode::Output { .. } => 2,
            OpCode::JumpIfTrue { .. } => 3,
            OpCode::Add { .. }
            | OpCode::Mult { .. }
            | OpCode::JumpIfFalse { .. }
            | OpCode::LessThan { .. }
            | OpCode::Equals { .. } => 4,
        }
    }

    /// Read the next `OpCode` start at position `ip`
    pub fn next(ip: &mut usize, slice: &[Int]) -> Result<OpCode> {
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
                destination: next_int_code()?,
            },
            6 => OpCode::JumpIfFalse {
                test: next_int_code()?,
                destination: next_int_code()?,
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
            _ => unreachable!(
                "Received unrecognised instruction: {} at index: {}",
                instruction, *ip
            ),
        })
    }
}
