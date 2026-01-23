//! Remove command implementation

use crate::cli::commands::AppContext;
use crate::utils::error::{Error, Result};

/// Execute the remove command
pub async fn execute(skill_name: &str, force: bool) -> Result<()> {
    let ctx = AppContext::new()?;

    // Check if skill exists
    use crate::services::SkillService;
    let skill = ctx
        .skill_service
        .get(skill_name)
        .await?
        .ok_or_else(|| Error::SkillNotFound(skill_name.to_string()))?;

    // Confirm removal unless force flag is set
    if !force {
        println!("About to remove skill: {}", skill.name);
        println!("  Source: {}", skill.source);
        println!("  Scope: {}", skill.scope);
        println!();
        print!("Are you sure? [y/N] ");

        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    // Remove the skill
    ctx.skill_service.remove(skill_name).await?;

    println!("✓ Removed skill: {}", skill_name);
    Ok(())
}
