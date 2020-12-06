use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
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
        .error_for_status()?
        .text()
        .await?)
}

pub async fn get_description(client: &Client, year: usize, day: usize) -> Result<String> {
    let url = format!("{}/{year}/day/{day}", BASE_URL, year = year, day = day);
    let html = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse(r#".day-desc"#).expect("failed to init html selector");

    let mut description = format!("// See: {}\n", url);
    for element in document.select(&selector) {
        let text = html2text::from_read(&element.html().as_bytes()[..], 100);
        for line in text.lines().map(|l| l.trim()) {
            if line.len() > 0 {
                description.push_str(&format!("// {}\n", line));
            } else {
                description.push_str("//\n");
            }
        }
    }

    description.push('\n');
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
        .await?
        .error_for_status()?;

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

pub async fn create_or_update_challenge(client: &Client, year: usize, day: usize) -> Result<()> {
    // create input file if it didn't exist
    let input_file = format!("examples/{day}.txt", day = day);
    if let Ok(mut f) = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&input_file)
    {
        match get_input(client, year, day).await {
            Ok(input) => f.write_all(input.as_bytes())?,
            Err(e) => {
                drop(f);
                fs::remove_file(&input_file)?;
                return Err(e);
            }
        }
    }

    // create or update rust source file
    let description = get_description(client, year, day).await?;
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&format!("examples/{day}.rs", day = day))
    {
        // file already existed with data, so remove the first comment (puzzle description) and re-write it
        if f.metadata()?.len() > 0 {
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;

            let mut updated = String::from(description);
            contents
                .lines()
                .skip_while(|line| line.starts_with("//"))
                .for_each(|line| updated.push_str(&format!("{}\n", line)));
            f.seek(SeekFrom::Start(0))?;
            f.write_all(updated.as_bytes())?;
        } else {
            f.write_all(new_source_file(description.as_str(), day).as_bytes())?;
        }
    }

    Ok(())
}

fn new_source_file(description: &str, day: usize) -> String {
    format!(
        r#"{description}

fn main() {{
    let input = include_str!("./{day}.txt");

    // ...

    aoc_lib::set_part_1!(0);
    // aoc_lib::set_part_2!(0);
}}"#,
        description = description,
        day = day
    )
}
