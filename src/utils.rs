pub fn string_split2<'a>(pattern: &'a str, string: &'a str) -> (&'a str, &'a str) {
    let parts = string.split(pattern).collect::<Vec<&str>>();
    (parts[0], parts[1])
}
