use crate::Int;

pub trait IntWrite {
    fn int_write(&mut self, value: Int);
}

impl IntWrite for &mut Vec<Int> {
    fn int_write(&mut self, value: Int) {
        self.push(value);
    }
}
