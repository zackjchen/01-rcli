use crate::{process::process_http_serve, CmdExcuter};
use clap::Parser;
use std::path::PathBuf;

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[clap(name = "serve", about = "Serve a http server")]
    Serve(HttpServeOpts),
}
impl CmdExcuter for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => {
                eprintln!("opts: {:?}", &opts);
                process_http_serve(opts.dir, opts.port).await?;
            }
        };
        Ok(())
    }
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
