//!  rcli csv -i input.csv -o output.json --header -d ','
//! -i输入文件 -o输出文件 --header是否包含表头 -d分隔符

use std::fs;

use clap::Parser;
use rcli::{
    cli::{
        http::HttpSubCommand,
        text::{TextSignFormat, TextSubCommand},
        Base64SubCommand, Opts, SubCommand,
    },
    process::{
        process_csv, process_decode, process_encode, process_genpass, process_http_serve,
        process_text_generate, process_text_sign, process_text_verify,
    },
};
use zxcvbn::zxcvbn;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = &opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            eprintln!("opts: {:?}", &opts);
            process_csv(
                &opts.input,
                &output,
                opts.delimiter,
                opts.header,
                opts.format,
            )?;
        }
        SubCommand::GenPass(opts) => {
            eprintln!("opts: {:?}", &opts);
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            // 不要换行, 不然save的时候会有换行
            print!("{}", password);
            // output the password strength
            let estimate = zxcvbn(&password, &[]).unwrap();
            // 这种print的方式是为了在 rcli genpass -> output.txt 时候，只保存密码, 不保存这个信息
            eprintln!("password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                eprintln!("opts: {:?}", &opts);
                let encoded = process_encode(&opts.input, opts.format)?;
                eprintln!("{}", encoded)
            }
            Base64SubCommand::Decode(opts) => {
                eprintln!("opts: {:?}", &opts);
                let decoded = process_decode(&opts.input, opts.format)?;
                eprintln!("{}", decoded)
            }
        },
        SubCommand::Text(subcmd) => {
            eprintln!("subcmd: {:?}", &subcmd);
            match subcmd {
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
                    }
                }
            }
        }
        SubCommand::Http(subcmd) => {
            match subcmd {
                HttpSubCommand::Serve(opts) => {
                    // eprintln!("Serving as http://0.0.0.0:{:?}", &opts.port);
                    process_http_serve(opts.dir, opts.port).await?;
                }
            }
        }
    }
    Ok(())
}
