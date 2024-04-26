use super::{verify_file, verify_path};
use crate::{
    process::{
        process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
        process_text_verify,
    },
    CmdExcuter,
};
use clap::Parser;
use std::{fmt::Display, fs, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[clap(name = "sign", about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[clap(name = "verify", about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[clap(name = "generate", about = "Generate a key for signing")]
    Generate(TextKeyGenerateOpts),
    #[clap(name = "encrypt", about = "encrypt a message with a key")]
    Encrypt(TextEncryptOpts),
    #[clap(name = "decrypt", about = "decrypt a message with a key")]
    Decrypt(TextDecryptOpts),
}
impl CmdExcuter for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        eprintln!("subcmd: {:?}", &self);
        match self {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                print!("{:x?}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified =
                    process_text_verify(&opts.input, &opts.key, &opts.signiture, opts.format)?;
                eprintln!("verified: {}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let path = opts.output.join("blake3.txt");
                        fs::write(path, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        // 不能写入不存在的dir
                        let path = &opts.output;
                        fs::write(path.join("ed25519.sk"), &key[0])?;
                        fs::write(path.join("ed25519.pk"), &key[1])?;
                    }
                    TextSignFormat::ChaCha20 => {
                        let path = opts.output.join("chacha20.txt");
                        fs::write(path, &key[0])?;
                    }
                }
            }
            TextSubCommand::Encrypt(opts) => {
                let encrypt = process_text_encrypt(&opts.input, &opts.key)?;
                print!("{}", encrypt)
            }
            TextSubCommand::Decrypt(opts) => {
                let decrypt = process_text_decrypt(&opts.input, &opts.key)?;
                print!("{}", decrypt)
            }
        }
        Ok(())
    }
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
#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    /// 输入文件路径, 默认stdin
    #[clap(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    /// 私钥文件路径
    #[clap(short, long, value_parser=verify_file)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    /// 输出文件路径
    #[clap(short, long, value_parser=verify_file, default_value = "-")]
    pub input: String,
    /// 私钥文件路径
    #[clap(short, long, value_parser=verify_file)]
    pub key: String,
}
#[derive(Debug, Copy, Clone, Parser)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
    ChaCha20,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    /// optional: [blake3, ed25519, chacha20] default: blake3
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
            "chacha20" => Ok(TextSignFormat::ChaCha20),
            _ => Err("Invalid format"),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
            TextSignFormat::ChaCha20 => "chacha20",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
