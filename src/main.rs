//!  rcli csv -i input.csv -o output.json --header -d ','
//! -i输入文件 -o输出文件 --header是否包含表头 -d分隔符

use clap::Parser;
use rcli::opts::{Opts, SubCommand};
use rcli::process;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            process::process_csv(&opts.input, &opts.output, opts.delimiter, opts.header)?;
        }
    }
    Ok(())
}
