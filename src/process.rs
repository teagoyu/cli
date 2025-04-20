use csv::Reader;
use anyhow::Result;
use serde::{Deserialize, Serialize};

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


pub fn process_csv(input:&str,output:&str) -> Result<()>{
    let mut readers = Reader::from_path(input)?;
    let mut ret: Vec<Player> = Vec::with_capacity(128);
    for result in readers.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }
    serde_json::to_writer_pretty(
        std::fs::File::create(output)?,
        &ret,
    )?;
    Ok(())
}
