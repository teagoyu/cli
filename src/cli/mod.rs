mod csv;
mod genpass;

use crate::cli::csv::CsvOpts;
use clap::Parser;

pub use self::csv::OutputFormat;
pub use self::genpass::GenPassOpts;

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv,Convert CSV TO JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate a rand password")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Parser)]
#[command(name = "rcli",version,author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
