use std::fs::OpenOptions;
use std::io::Write;
use std::string::ToString;

use anyhow::Result;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};
use scraper::{Html, Selector};

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

pub async fn get_description(client: &Client, year: usize, day: usize) -> Result<String> {
    let html = client
        .get(&format!(
            "{}/{year}/day/{day}",
            BASE_URL,
            year = year,
            day = day
        ))
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse(r#".day-desc"#).expect("failed to init html selector");

    let mut description = String::new();
    for element in document.select(&selector) {
        let text = html2text::from_read(&element.html().as_bytes()[..], 100);
        for line in text.lines() {
            description.push_str(&format!("// {}\n", line));
        }
    }

    Ok(description)
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
        .write_all(get_input(client, year, day).await?.as_bytes())?;

    // write rust source file
    let description = get_description(client, year, day).await?;
    OpenOptions::new()
        .create_new(true)
        .truncate(false)
        .write(true)
        .open(format!("examples/{day}.rs", day = day))?
        .write_all(new_source_file(description.as_str(), year, day).as_bytes())?;

    Ok(())
}

fn new_source_file(description: &str, year: usize, day: usize) -> String {
    format!(
        r#"// See: {base_url}/{year}/day/{day}
{description}
fn main() {{
    let input = include_str!("./{day}.txt");

    // ...

    aoc_lib::set_part_1!(0);
    // aoc_lib::set_part_2!(0);
}}"#,
        base_url = BASE_URL,
        description = description,
        year = year,
        day = day
    )
}
