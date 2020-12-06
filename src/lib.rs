pub mod aoc;
pub mod args;
pub mod credentials;

use paste::paste;

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

            #[macro_export]
            macro_rules! [<get_ $ident>] {
                () => {{
                    std::fs::read_to_string(concat!("/tmp/aoc-{}", stringify!($ident)))
                }};
            }

            #[macro_export]
            macro_rules! [<$ident _complete>] {
                () => {{
                    use aoc_lib::[<get_ $ident>];
                    match [<get_ $ident>]!() {
                        Ok(s) => Ok(s.contains("--- Part Two ---")),
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::NotFound {
                                Ok(false)
                            } else {
                                Err(e)
                            }
                        }
                    }
                }};
            }

            #[macro_export]
            macro_rules! [<remove_ $ident>] {
                () => {{
                    let path = std::path::PathBuf::from(concat!("/tmp/aoc-{}", stringify!($ident)));
                    if path.exists() {
                        std::fs::remove_file(&path)?;
                    }
                }};
            }

            #[macro_export]
            macro_rules! [<submit_ $ident>] {
                ($client:expr, $year:expr, $day:expr) => {{
                    use aoc_lib::[<get_ $ident>];
                    use aoc_lib::aoc::{self, AocPart};
                    match [<get_ $ident>]!() {
                        Ok(answer) => {
                            println!("Submitting {} answer: '{}'...", stringify!($ident), &answer);
                            aoc::submit_answer($client, $year, $day, AocPart::One, answer.as_str()).await?;
                        },
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::NotFound {
                                eprintln!("Not submitting {} since it doesn't exist", stringify!($ident));
                            } else {
                                eprintln!("Error submitting {}: {}", stringify!($ident), e);
                            }
                        }
                    }
                }};
            }
        }
    };
}

define_aoc_macro!(part_1);
define_aoc_macro!(part_2);
