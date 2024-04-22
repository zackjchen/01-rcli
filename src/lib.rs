pub mod cli;
pub mod process;
pub mod utils;
use anyhow::Result;

/// because of http command is async， if want to execute it , this function need async
#[allow(async_fn_in_trait)]
pub trait CmdExcuter {
    // fn execute(self) -> impl std::future::Future<Output = Result<()>> + Send;
    async fn execute(self) -> Result<()>;
}
