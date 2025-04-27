use super::verify_input_file;
use clap::Parser;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, default_value = "json",value_parser=parse_format)]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

pub fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format
        .parse::<OutputFormat>()
        .map_err(|_| anyhow::anyhow!("Invalid format"))
}
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
impl From<&str> for OutputFormat {
    fn from(format: &str) -> Self {
        match format.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "yaml" => OutputFormat::Yaml,
            _ => panic!("Invalid format"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    /// 字符串转 OutputFormat
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Into::<&'static str>::into(*self).fmt(f)
    }
}
