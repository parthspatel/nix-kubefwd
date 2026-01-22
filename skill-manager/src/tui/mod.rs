//! TUI module for Claude Skill Manager

pub mod app;
pub mod screens;
pub mod widgets;

pub use app::App;

use crate::utils::error::Result;

/// Run the TUI application
pub async fn run(_section: Option<&str>) -> Result<()> {
    // TODO: Implement TUI
    println!("TUI not yet implemented");
    Ok(())
}
