//! CLI command implementations

pub mod add;
pub mod app;
pub mod completions;
pub mod config;
pub mod conflicts;
pub mod create;
pub mod doctor;
pub mod edit;
pub mod enable;
pub mod export;
pub mod import;
pub mod init;
pub mod list;
pub mod migrate;
pub mod remove;
pub mod search;
pub mod show;
pub mod sync;
pub mod update;

pub use app::AppContext;

use crate::utils::error::Result;

/// Trait for CLI commands
pub trait Command {
    /// Execute the command
    fn execute(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}
