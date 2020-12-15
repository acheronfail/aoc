use std::io::{self, Write};

use anyhow::Result;

pub fn string_split2<'a>(pattern: &'a str, string: &'a str) -> (&'a str, &'a str) {
    let parts = string.split(pattern).collect::<Vec<&str>>();
    (parts[0], parts[1])
}

#[inline]
pub fn digit_at(input: usize, pos: usize) -> usize {
    (input / 10_usize.pow(pos as u32)) % 10
}

pub fn prompt_from_stdin(prompt: Option<&str>) -> Result<String> {
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush()?;
    }

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;

    Ok(answer.trim().to_string())
}
