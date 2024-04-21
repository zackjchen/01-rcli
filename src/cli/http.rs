use std::path::PathBuf;

use clap::Parser;

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[clap(name = "serve", about = "Serve a http server")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    /// port,default is 8080
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    /// 提供的静态资源文件路径
    #[clap(short, long, default_value = ".", value_parser = verify_path)]
    pub dir: PathBuf,
}
