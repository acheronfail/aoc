use super::Int;

#[derive(Debug, Copy, Clone)]
pub enum IntCode {
    Position(usize),
    Immediate(Int),
    Relative(Int),
}

impl IntCode {
    pub fn as_int(self, memory: &[Int], rb: usize) -> Int {
        match self {
            IntCode::Immediate(x) => x,
            IntCode::Position(i) => memory[i],
            IntCode::Relative(offset) => memory[((rb as Int) + offset) as usize],
        }
    }

    pub fn as_address(self, rb: usize) -> Option<usize> {
        match self {
            IntCode::Position(i) => Some(i),
            IntCode::Relative(offset) => Some(((rb as Int) + offset) as usize),
            IntCode::Immediate(_) => None,
        }
    }
}
