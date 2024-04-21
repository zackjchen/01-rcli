# RCLI

rcli is a rust CLI tool

## Clap
clap 有两种写cli的方式，
第一种是builder的方式
第二种是derive的方式

```rust
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    /// arg代表可以以选择config, 用 -或者-- 方式使用
    /// short 代表简写，去第一个字母 -c, long代表全写， --config
    /// value_name代表接受参数的名字
    /// value_parser对接受参数的校验方法
    #[arg(short, long, value_name = "FILE", value_parser=fn)]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// 子命令，一般使用枚举作为子命令
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}

```


## 代码提交
主分支
pre-commit install
git add .
git commit -a


其它分支
git checkout -b branch-name // 新建并切换，去掉-b切换到已有分支
git add .
git commit -a
git branch -D new-features// 删除分支

提交一个tag
git tag -a v1-base64
git push -u origin v1-base64


##  !注
文件save的时候 command+shift+p 选择 save without formatting
否则会多出一行


vscode 插件 rest client
作用: 可以直接写http请求做测试
文件后缀: .rest


Path和PathBuf
Path: 相当于&str
PathBuf: 相当于具有ownership的String
