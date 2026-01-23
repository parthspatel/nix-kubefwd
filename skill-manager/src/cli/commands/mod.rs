//! CLI command implementations

pub mod add;
pub mod init;
pub mod list;
pub mod migrate;

use crate::utils::error::Result;

/// Trait for CLI commands
pub trait Command {
    /// Execute the command
    fn execute(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}
