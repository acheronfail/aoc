use clap::AppSettings::ColoredHelp;
use clap::Clap;

#[derive(Clap)]
#[clap(global_setting(ColoredHelp))]
pub struct Args {
    #[clap(subcommand)]
    pub command: ArgCommand,
}

#[derive(Clap)]
pub enum ArgCommand {
    New {
        #[clap(short = 'y', long = "year")]
        year: usize,
        #[clap(short = 'd', long = "day")]
        day: usize,
    },
}

impl Args {
    pub fn parse() -> Args {
        <Args as Clap>::parse()
    }
}
