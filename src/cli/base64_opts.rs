use std::{
    fmt::{self, Display},
    str::FromStr,
};

use super::verify_input_file;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[clap(name = "encode", about = "Encode base64 string")]
    Encode(Base64EncodeOpts),
    #[clap(name = "decode", about = "Decode base64 string")]
    Decode(Base64DecodeOpts),
}
#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    /// 输入文件路径， 默认值'-'代表从标准输入读取
    #[clap(short, long, value_parser=verify_input_file, default_value = "-")]
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