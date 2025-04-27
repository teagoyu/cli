use super::verify_input_file;
use clap::Parser;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(about = "Encode a string to base64", name = "encode")]
    Encode(Base64EncodeOpts),
    #[command(about = "Decode a base64 string", name = "decode")]
    Decode(Base64DecodeOpts),
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file,default_value = "-")]
    pub input: String,

    #[arg(long,default_value = "standard",value_parser=parse_base64_format)]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser = verify_input_file,default_value = "-")]
    pub input: String,
    #[arg(long,default_value = "standard",value_parser=parse_base64_format)]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}
fn parse_base64_format(format: &str) -> anyhow::Result<Base64Format> {
    match format {
        "standard" => Ok(Base64Format::Standard),
        "urlsafe" => Ok(Base64Format::UrlSafe),
        _ => anyhow::bail!("Invalid base64 format"),
    }
}

impl FromStr for Base64Format {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
