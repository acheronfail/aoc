pub fn string_split2<'a>(pattern: &'a str, string: &'a str) -> (&'a str, &'a str) {
    let parts = string.split(pattern).collect::<Vec<&str>>();
    (parts[0], parts[1])
}

#[inline]
pub fn digit_at(input: usize, pos: usize) -> usize {
    (input / 10_usize.pow(pos as u32)) % 10
}
