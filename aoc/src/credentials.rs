use anyhow::{anyhow, Result};
use clap::crate_name;

const KEYTAR_SERVICE_NAME: &str = crate_name!();
const KEYTAR_SERVICE_ACCOUNT: &str = "aoc";

pub fn get_session_token() -> Result<String> {
    match keytar::find_password(KEYTAR_SERVICE_NAME) {
        Ok(result) if result.success => Ok(result.password),
        Ok(_) => {
            // No password was returned, prompt for one and then set it.
            if let Ok(pass) = rpassword::read_password_from_tty(Some("aoc session token: ")) {
                keytar::set_password(KEYTAR_SERVICE_NAME, KEYTAR_SERVICE_ACCOUNT, &pass)?;
                Ok(pass)
            } else {
                Err(anyhow!("failed prompting user for password"))
            }
        }
        Err(e) => Err(anyhow!("{}", e)),
    }
}
