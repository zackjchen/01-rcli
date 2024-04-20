use clap::{Parser, Subcommand};
use std::{
    fmt::{self, Display},
    path::Path,
    str::FromStr,
};

#[derive(Debug, Parser)]
#[clap(name = "rcli", version, author, about, long_about)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[clap(name = "csv", about = "Show csv, or convert to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,
    /// default_value默认值，传字符串然后由Parser convert
    #[arg(short, long, /*default_value = "output.json"*/)]
    pub output: Option<String>,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    /// default_value_t不需要转换直接写rust的类型
    #[arg(long, default_value_t = true)]
    pub header: bool,
    /// 输出文件的格式
    #[arg(long,value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}
impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这里的Into::<&'static str>是为了调用From的实现
        // 并且先调用了into返回&str，再调用的write!，所以这里的返回的&str生命周期足够长
        // write!(f, "{}", Into::<&'static str>::into(*self))
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found")
    }
}
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // 这里的范型是OutputFormat, 由于函数声明返回值指定了，这里可以省略
    format.parse()
}
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
