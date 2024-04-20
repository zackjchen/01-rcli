use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    /// password长度
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    /// 是否包含大写字母
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    /// 是否包含小写字母
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    /// 是否包含数字
    #[arg(short, long, default_value_t = true)]
    pub number: bool,
    /// 是否包含特殊字符
    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
}
