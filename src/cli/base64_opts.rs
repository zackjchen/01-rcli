use std::{
    fmt::{self, Display},
    str::FromStr,
};

use crate::{
    process::{process_decode, process_encode},
    CmdExcuter,
};

use super::verify_file;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[clap(name = "encode", about = "Encode base64 string")]
    Encode(Base64EncodeOpts),
    #[clap(name = "decode", about = "Decode base64 string")]
    Decode(Base64DecodeOpts),
}

impl CmdExcuter for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => {
                eprintln!("opts: {:?}", &opts);
                let encoded = process_encode(&opts.input, opts.format)?;
                eprintln!("{}", encoded)
            }
            Base64SubCommand::Decode(opts) => {
                eprintln!("opts: {:?}", &opts);
                let decoded = process_decode(&opts.input, opts.format)?;
                eprintln!("{}", decoded)
            }
        }
        Ok(())
    }
}
#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    /// 输入文件路径， 默认值'-'代表从标准输入读取
    #[clap(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    /// format, optional: [standard, urlsafe]
    #[clap(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    /// 输入文件路径， 默认值'-'代表从标准输入读取
    #[clap(short, long, default_value = "-")]
    pub input: String,
    /// format, optional: [standard, urlsafe]
    #[clap(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, &'static str> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err("Invalid format"),
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
