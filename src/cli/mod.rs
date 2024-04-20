use std::path::Path;

use clap::{Parser, Subcommand};
pub mod base64_opts;
pub mod csv_opts;
pub mod genpass_opts;
pub use base64_opts::Base64SubCommand;
pub use csv_opts::CsvOpts;
pub use genpass_opts::GenPassOpts;

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
    #[clap(name = "genpass", about = "generate random password")]
    GenPass(GenPassOpts),
    #[clap(
        name = "base64",
        about = "support base64 encode and decode, 从键盘录入需要回车后ctrl+D结束输入",
        subcommand
    )]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() || filename == "-" {
        Ok(filename.to_string())
    } else {
        Err("File not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        let filename = "Cargo.toml";
        assert_eq!(verify_input_file(filename), Ok(filename.to_string()));
        let filename = "Cargo.toml1";
        assert_eq!(verify_input_file(filename), Err("File not found"));
        let filename = "-";
        assert_eq!(verify_input_file(filename), Ok(filename.to_string()));
    }
}
