use clap::{crate_authors, Parser};

#[derive(Parser)]
#[clap(author = crate_authors!())]
pub struct Args {
    pub year: usize,
    pub day: usize,

    #[clap(short = 'r', long = "release")]
    pub release: bool,
}

impl Args {
    pub fn parse() -> Args {
        <Args as Parser>::parse()
    }
}
