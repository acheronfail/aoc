use super::Int;

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

pub trait IntWrite {
    fn int_write(&mut self, value: Int);
}

impl IntWrite for &mut Vec<Int> {
    fn int_write(&mut self, value: Int) {
        self.push(value);
    }
}
