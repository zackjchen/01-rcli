pub mod cli;
pub mod process;
pub mod utils;
use anyhow::Result;

/// because of http command is async， if want to execute it , this function need async
pub trait CmdExcuter {
    #[allow(dead_code)]
    fn execute(self) -> impl std::future::Future<Output = Result<()>> + Send;
}
