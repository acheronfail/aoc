use super::Int;

#[derive(Debug, Copy, Clone)]
pub enum IntCode {
    Position(usize),
    Immediate(Int),
}

impl IntCode {
    pub fn as_int(&self, memory: &[Int]) -> Int {
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
