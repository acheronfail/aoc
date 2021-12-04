#![allow(unused)]

use anyhow::{anyhow, Result};
use std::io::{Read, Write};

pub mod int_code;
pub mod io;
pub mod op_code;
pub mod program;

pub use program::{Program, StopReason};

use int_code::IntCode;
use io::{IntRead, IntWrite};
use op_code::OpCode;

pub type Int = i64;

pub fn ints_from_str(s: &str) -> Vec<Int> {
    s.split(',')
        .map(|n| n.trim().parse::<Int>().unwrap())
        .collect::<Vec<Int>>()
}
