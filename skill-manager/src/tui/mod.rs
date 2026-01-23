//! TUI module for Claude Skill Manager

pub mod app;
pub mod screens;
pub mod widgets;

pub use app::App;

use crate::utils::error::Result;

/// Run the TUI application
pub async fn run(section: Option<&str>) -> Result<()> {
    println!("Claude Skill Manager - Interactive UI");
    println!();
    println!("The TUI is not yet fully implemented.");
    println!();
    println!("For now, please use the CLI commands:");
    println!("  csm list          - List all skills");
    println!("  csm add <source>  - Add a skill");
    println!("  csm show <skill>  - Show skill details");
    println!("  csm enable <skill> - Enable a skill");
    println!("  csm disable <skill> - Disable a skill");
    println!("  csm remove <skill> - Remove a skill");
    println!("  csm search <query> - Search skills");
    println!("  csm update        - Check for updates");
    println!("  csm conflicts     - Detect conflicts");
    println!("  csm sync          - Sync skill state");
    println!("  csm doctor        - Diagnose issues");
    println!();

    if let Some(s) = section {
        println!("(Requested section: {})", s);
    }

    println!("Run 'csm --help' for full command list.");

    Ok(())
}
