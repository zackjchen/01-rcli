//!  rcli csv -i input.csv -o output.json --header -d ','
//! -i输入文件 -o输出文件 --header是否包含表头 -d分隔符

use clap::Parser;
use rcli::opts::{Opts, SubCommand};
use rcli::process::process_csv;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = &opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            println!("opts: {:?}", &opts);
            process_csv(
                &opts.input,
                &output,
                opts.delimiter,
                opts.header,
                opts.format,
            )?;
        }
    }
    Ok(())
}
