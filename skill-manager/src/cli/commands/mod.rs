//! CLI command implementations

pub mod init;
pub mod add;
pub mod list;

use crate::utils::error::Result;

/// Trait for CLI commands
pub trait Command {
    /// Execute the command
    fn execute(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}
