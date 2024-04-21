use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
pub mod base64_opts;
pub mod csv_opts;
pub mod genpass_opts;
pub mod http;
pub mod text;
use self::{http::HttpSubCommand, text::TextSubCommand};
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
    // 子命令和args用 /// 注释，用于生成帮助文档
    /// text subcommand, support text cryptographic hash
    #[clap(subcommand)]
    Text(TextSubCommand),
    // serve http server
    #[clap(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found")
    }
}
fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = PathBuf::from(path);
    if p.exists() && p.is_dir() {
        Ok(p)
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        let filename = "./Cargo.toml";
        assert_eq!(verify_file(filename), Ok(filename.to_string()));
        let filename = "./Cargo.toml1";
        assert_eq!(verify_file(filename), Err("File not found"));
        let filename = "-";
        assert_eq!(verify_file(filename), Ok(filename.to_string()));
    }
}
