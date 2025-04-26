use crate::cli::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: String, output: String, output_format: OutputFormat) -> Result<()> {
    let mut readers = Reader::from_path(input)?;
    let mut ret: Vec<Value> = Vec::with_capacity(128);
    let mut ret_toml: Vec<HashMap<String, String>> = Vec::with_capacity(128);
    let header = readers.headers()?.clone();
    for result in readers.records() {
        let record = result?;
        // 将每一行记录转换为 HashMap<String, String>
        let json_value = header
            .iter()
            .zip(record.iter())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Value>();

        println!("{:?}", record);
        ret.push(json_value);
        let toml_value = header
            .iter()
            .zip(record.iter())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<String, String>>();
        ret_toml.push(toml_value);
        // let record: Player = result?;
        // ret.push(record);
    }
    let content = match output_format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    // serde_json::to_writer_pretty(std::fs::File::create(output)?, &ret)?;
    Ok(())
}
