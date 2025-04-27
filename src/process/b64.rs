use crate::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};
use std::fs::File;
use std::io::Read;

// Read input from stdin or file
fn read_input(input: String) -> anyhow::Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

// Process encoded input
pub fn process_encode(input: String, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_input(input)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE.encode(buf),
    };
    println!("{}", encoded);
    Ok(())
}

// Process decoded input
pub fn process_decode(input: String, _format: Base64Format) -> anyhow::Result<()> {
    let buf = read_input(input)?;
    let decoded = match _format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE.decode(buf)?,
    };
    println!("{}", String::from_utf8(decoded)?);
    Ok(())
}
