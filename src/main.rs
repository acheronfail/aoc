use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use aoc_lib::aoc::AocPart;
use aoc_lib::args::Args;
use reqwest::Client;

fn prompt_from_stdin(prompt: Option<&str>) -> Result<String> {
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush()?;
    }

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;

    Ok(answer.trim().to_string())
}

async fn run_loop(client: &Client, year: usize, day: usize) -> Result<Option<AocPart>> {
    // create new challenge if it doesn't exist
    println!("Loading challenge {year}-{day}...", year = year, day = day);
    aoc_lib::aoc::create_or_update_challenge(&client, year, day).await?;

    // clean up old answers
    aoc_lib::remove_part_1!();
    aoc_lib::remove_part_2!();

    // catch ^C and kill watch loop
    let running = Arc::new(AtomicBool::new(true));
    let running_ctrlc = running.clone();
    // ignore errors (fails if we try to set it more than once)
    let _ = ctrlc::set_handler(move || running_ctrlc.store(false, Ordering::SeqCst));

    // start a watch/run loop
    println!("Starting watch loop...");
    let mut child = Command::new("cargo")
        .args(&["watch", "-x", &format!("run --example {day}", day = day)])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    loop {
        thread::sleep(Duration::from_secs(1));
        if !running.load(Ordering::SeqCst) {
            println!("\rStopping watch loop...");
            child.kill()?;
            break;
        }
    }

    // prompt to submit answers
    let answer = prompt_from_stdin(Some("Submit answers? [Y/n]: "))?;
    if matches!(answer.as_str(), "" | "y" | "yes" | "Y" | "YES") {
        if aoc_lib::part_1_complete!()? {
            aoc_lib::submit_part_2!(&client, year, day);
            Ok(Some(AocPart::Two))
        } else {
            aoc_lib::submit_part_1!(&client, year, day);
            Ok(Some(AocPart::One))
        }
    } else {
        Ok(None)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = aoc_lib::aoc::get_client()?;

    if matches!(
        run_loop(&client, args.year, args.day).await?,
        Some(AocPart::One)
    ) {
        run_loop(&client, args.year, args.day).await?;
    }

    Ok(())
}
