use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use aoc_lib::args::{ArgCommand, Args};

fn prompt_from_stdin(prompt: Option<&str>) -> Result<String> {
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush()?;
    }

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;

    Ok(answer.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = aoc_lib::aoc::get_client()?;

    match args.command {
        ArgCommand::New { year, day } => {
            // create new challenge if it doesn't exist
            match aoc_lib::aoc::new_challenge(&client, year, day).await {
                Ok(_) => println!("Created: {year}-{day}", year = year, day = day),
                Err(e) => eprintln!("Failed to create new puzzle: {}", e),
            }

            // catch ^C and kill watch loop
            let running = Arc::new(AtomicBool::new(true));
            let running_ctrlc = running.clone();
            ctrlc::set_handler(move || running_ctrlc.store(false, Ordering::SeqCst))
                .expect("Error setting Ctrl-C handler");

            // start a watch/run loop
            let mut child = Command::new("cargo")
                .args(&["watch", "-x", &format!("run --example {day}", day = day)])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;

            while running.load(Ordering::SeqCst) {}
            println!("\rStopping watch loop...");
            child.kill()?;

            // prompt to submit answers
            let answer = prompt_from_stdin(Some("Submit answers?: "))?;
            if matches!(answer.as_str(), "y" | "yes" | "Y" | "YES") {
                aoc_lib::submit_answers(&client, year, day).await?;
            }
        }
    }

    Ok(())
}
