//!  rcli csv -i input.csv -o output.json --header -d ','
//! -i输入文件 -o输出文件 --header是否包含表头 -d分隔符

use clap::Parser;
use rcli::{cli::Opts, CmdExcuter};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();

    let cmd = opts.cmd;
    cmd.execute().await?;
    Ok(())
}
