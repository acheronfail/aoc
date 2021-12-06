use anyhow::{anyhow, Result};
use clap::crate_name;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;

lazy_static! {
    static ref PROJECT_DIRS: ProjectDirs =
        ProjectDirs::from("fail.acheron", "", crate_name!()).unwrap();
    static ref SESSION_FILE: PathBuf = PathBuf::from(PROJECT_DIRS.cache_dir()).join("session.aoc");
}

fn find_password() -> Result<String> {
    let mut s = String::new();
    OpenOptions::new()
        .create(false)
        .read(true)
        .open(SESSION_FILE.as_path())?
        .read_to_string(&mut s)?;

    Ok(s)
}

fn set_password(pass: impl AsRef<str>) -> Result<()> {
    fs::create_dir_all(PROJECT_DIRS.cache_dir())?;
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(SESSION_FILE.as_path())?
        .write_all(pass.as_ref().as_bytes())?;

    Ok(())
}

pub fn get_session_token() -> Result<String> {
    match find_password() {
        Ok(token) => Ok(token),
        Err(e) => match e.downcast_ref::<std::io::Error>() {
            Some(e) => match e.kind() {
                ErrorKind::NotFound => {
                    // No password was returned, prompt for one and then set it.
                    if let Ok(pass) = rpassword::read_password_from_tty(Some("aoc session token: "))
                    {
                        println!("token: {}", pass);
                        dbg!(set_password(&pass))?;
                        Ok(pass)
                    } else {
                        Err(anyhow!("failed prompting user for password"))
                    }
                }
                _ => Err(anyhow!("Failed to read file!")),
            },
            None => Err(anyhow!("Failed to read token file!")),
        },
    }
}
