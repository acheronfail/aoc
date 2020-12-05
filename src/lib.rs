pub mod aoc;
pub mod args;
pub mod credentials;

use anyhow::Result;
use paste::paste;
use reqwest::Client;

use aoc::AocPart;

macro_rules! define_aoc_macro {
    ($ident:ident) => {
        paste! {
            #[macro_export]
            macro_rules! [<set_ $ident>] {
                ($result:expr) => {{
                    use std::fs::OpenOptions;
                    use std::io::Write;

                    println!("{}: {}", stringify!($ident), $result);
                    OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(concat!("/tmp/aoc-{}", stringify!($ident)))
                        .unwrap()
                        .write_all(format!("{}", $result).as_bytes())
                        .unwrap();
                }};
            }

            macro_rules! [<get_ $ident>] {
                () => {{
                    std::fs::read_to_string(concat!("/tmp/aoc-{}", stringify!($ident)))
                }};
            }

            macro_rules! [<submit_ $ident>] {
                ($client:expr, $year:expr, $day:expr) => {{
                    let answer = [<get_ $ident>]!()?;
                    println!("Submitting part 1 answer: '{}'...", &answer);
                    aoc::submit_answer($client, $year, $day, AocPart::One, answer.as_str()).await?;
                }};
            }
        }
    };
}

define_aoc_macro!(part_1);
define_aoc_macro!(part_2);

pub async fn submit_answers(client: &Client, year: usize, day: usize) -> Result<()> {
    submit_part_1!(client, year, day);
    submit_part_2!(client, year, day);

    Ok(())
}
