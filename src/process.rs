use std::fs;

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
// 这一句是用来匹配文件里面的字段名和结构体里面的字段名
pub struct Row {
    col1: String,
    col2: String,
    #[serde(rename = "col3")]
    age: u8,
}

pub fn process_csv(input: &str, output: &str, delimiter: char, header: bool) -> anyhow::Result<()> {
    // let mut reader = Reader::from_path(input)?;
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(header)
        .from_path(input)?;

    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let row: Row = result?;
        // println!("{:?}", row);
        ret.push(row);
    }

    fs::write(output, serde_json::to_string_pretty(&ret)?)?;
    Ok(())
}
