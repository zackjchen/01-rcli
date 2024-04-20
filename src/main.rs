//!  rcli csv -i input.csv -o output.json --header -d ','
//! -i输入文件 -o输出文件 --header是否包含表头 -d分隔符

use clap::Parser;
use rcli::{
    cli::{Base64SubCommand, Opts, SubCommand},
    process::{process_csv, process_decode, process_encode, process_genpass},
};

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
        SubCommand::GenPass(opts) => {
            println!("opts: {:?}", &opts);
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                eprintln!("opts: {:?}", &opts);
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                eprintln!("opts: {:?}", &opts);
                process_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
