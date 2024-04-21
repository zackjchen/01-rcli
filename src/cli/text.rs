use super::verify_file;
use clap::Parser;
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[clap(name = "sign", about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[clap(name = "verify", about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[clap(name = "generate", about = "Generate a key for signing")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    /// 输入文件路径， 默认值'-'代表从标准输入读取
    #[clap(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    /// 私钥文件路径
    #[clap(short, long, value_parser=verify_file)]
    pub key: String,
    /// format, optional: [blake3, ed25519]
    #[clap(long, value_parser = parse_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

/// 非对称加密是用来验证数据来源和信息完整性的技术
/// 检测的是数据是否被篡改，而不是数据是否被窃取
#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    /// 输入文件路径， 默认值'-'代表从标准输入读取
    #[clap(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,

    /// key文件路径，
    #[clap(short, long)]
    pub key: String,

    /// 签名
    #[clap(short = 'S', long)]
    pub signiture: String,

    /// format, optional: [blake3, ed25519]
    #[clap(long, value_parser = parse_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Copy, Clone, Parser)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    /// optional: [blake3, ed25519]
    #[arg(short, long, default_value = "blake3", value_parser = parse_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

fn parse_sign_format(format: &str) -> Result<TextSignFormat, &'static str> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err("Invalid format"),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
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
