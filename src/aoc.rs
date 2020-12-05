use std::fs::OpenOptions;
use std::io::Write;
use std::string::ToString;

use anyhow::Result;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};

use crate::aoc;
use crate::credentials;

const BASE_URL: &str = "https://adventofcode.com";

#[derive(Copy, Clone, strum_macros::EnumString, strum_macros::ToString)]
pub enum AocPart {
    #[strum(serialize = "1")]
    One,
    #[strum(serialize = "2")]
    Two,
}

pub async fn get_input(client: &Client, year: usize, day: usize) -> Result<String> {
    Ok(client
        .get(&format!(
            "{}/{year}/day/{day}/input",
            BASE_URL,
            year = year,
            day = day
        ))
        .send()
        .await?
        .text()
        .await?)
}

pub async fn submit_answer(
    client: &Client,
    year: usize,
    day: usize,
    part: AocPart,
    answer: &str,
) -> Result<()> {
    let res = client
        .post(&format!(
            "{}/{year}/day/{day}/answer",
            BASE_URL,
            year = year,
            day = day
        ))
        .form(&[("level", part.to_string().as_str()), ("answer", answer)])
        .send()
        .await?;

    if res.status().is_success() {
        println!("Successfully submitted!");
    } else {
        println!("Error submitting! {:?}", res.status());
    }

    Ok(())
}

pub fn get_client() -> Result<Client> {
    let cookie = HeaderValue::from_str(&format!("session={}", credentials::get_session_token()?))?;
    let mut default_headers = HeaderMap::new();
    default_headers.insert(header::COOKIE, cookie);

    Ok(ClientBuilder::new()
        .default_headers(default_headers)
        .build()?)
}

pub async fn new_challenge(client: &Client, year: usize, day: usize) -> Result<()> {
    // write input file
    OpenOptions::new()
        .create_new(true)
        .truncate(true)
        .write(true)
        .open(format!("examples/{day}.txt", day = day))?
        .write_all(aoc::get_input(&client, year, day).await?.as_bytes())?;

    // write rust source file
    OpenOptions::new()
        .create_new(true)
        .truncate(false)
        .write(true)
        .open(format!("examples/{day}.rs", day = day))?
        .write_all(new_source_file(year, day).as_bytes())?;

    Ok(())
}

fn new_source_file(year: usize, day: usize) -> String {
    format!(
        r#"// AoC {year} {day}
fn main() {{
    let input = include_str!("./{day}.txt");
    println!("{{}}", input);
}}"#,
        year = year,
        day = day
    )
}
