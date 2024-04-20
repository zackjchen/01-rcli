use crate::cli::csv_opts::OutputFormat;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
// 这一句是用来匹配文件里面的字段名和结构体里面的字段名
pub struct Row {
    col1: String,
    col2: String,
    #[serde(rename = "col3")]
    age: u8,
}

pub fn process_csv(
    input: &str,
    output: &str,
    delimiter: char,
    header: bool,
    output_format: OutputFormat,
) -> anyhow::Result<()> {
    // let mut reader = Reader::from_path(input)?;
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(header)
        .from_path(input)?;

    let mut ret = Vec::with_capacity(128);
    // 不能两个mutable borrow
    let header = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let iter = header.iter().zip(record.iter());
        let row = match output_format {
            OutputFormat::Json => iter.collect::<Value>(),
            OutputFormat::Yaml => iter.collect::<Value>(),
        };
        // let row = .collect::<Value>();
        ret.push(row);
    }
    let content = match output_format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;

    Ok(())
}
