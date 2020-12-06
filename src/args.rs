use clap::AppSettings::ColoredHelp;
use clap::{crate_authors, Clap};

#[derive(Clap)]
#[clap(author = crate_authors!(), global_setting(ColoredHelp))]
pub struct Args {
    pub year: usize,
    pub day: usize,
}

impl Args {
    pub fn parse() -> Args {
        <Args as Clap>::parse()
    }
}
