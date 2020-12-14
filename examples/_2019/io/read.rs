use std::sync::{Arc, Mutex};

use crate::_2019::Int;

pub trait IntRead {
    fn int_read(&mut self) -> Option<Int>;
}

impl IntRead for &mut Vec<Int> {
    fn int_read(&mut self) -> Option<Int> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }
}

impl IntRead for Int {
    fn int_read(&mut self) -> Option<Int> {
        Some(*self)
    }
}

impl IntRead for Option<Int> {
    fn int_read(&mut self) -> Option<Int> {
        match self {
            Some(int) => int.int_read(),
            None => None,
        }
    }
}
