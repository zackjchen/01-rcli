use crate::{
    process::{process_jwt_sign, process_jwt_verify},
    CmdExcuter,
};
use clap::Parser;

#[derive(Debug, Parser)]
pub enum JwtSubCommand {
    #[clap(name = "sign", about = "Sign jwt, generate jwt token")]
    Sign(JwtSignOpts),
    #[clap(name = "verify", about = "verify jwt token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    /// subscribe name， eg: zackjchen@qq.com
    #[clap(short, long)]
    pub sub: String,

    /// expiration time,default 1, unit: hour
    #[clap(short, long, default_value_t = 1)]
    pub exp: u64,

    /// audience, company name, eg: ACME, HKJC, ...
    #[clap(short, long)]
    pub aud: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    /// encoded jwt token
    #[clap(short, long)]
    pub token: String,
}

impl CmdExcuter for JwtSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            JwtSubCommand::Sign(opts) => {
                println!("sign jwt: {:?}", opts);
                let token = process_jwt_sign(&opts.sub, opts.exp, &opts.aud)?;
                println!("{}", token);
            }
            JwtSubCommand::Verify(opts) => {
                println!("verify jwt: {:?}", opts);
                let res = process_jwt_verify(&opts.token)?;
                println!("verify pass, data:{}", res);
            }
        }
        Ok(())
    }
}
