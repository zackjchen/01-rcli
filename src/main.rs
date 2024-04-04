//!  rcli csv -i input.csv -o output.json --header -d ','
//! -i输入文件 -o输出文件 --header是否包含表头 -d分隔符

use clap::{Parser, Subcommand};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Parser)]
#[clap(name = "rcli", version, author, about, long_about)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}
#[derive(Debug, Subcommand)]
enum SubCommand {
    #[clap(name = "csv", about = "Show csv, or convert to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    input: String,
    /// default_value默认值，传字符串然后由Parser convert
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    /// default_value_t不需要转换直接写rust的类型
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
// 这一句是用来匹配文件里面的字段名和结构体里面的字段名
struct Row {
    col1: String,
    col2: String,
    #[serde(rename = "col3")]
    age: u8,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let row: Row = result?;
                // println!("{:?}", row);
                ret.push(row);
            }

            fs::write(opts.output, serde_json::to_string_pretty(&ret)?)?;
        }
    }

    Ok(())
}
