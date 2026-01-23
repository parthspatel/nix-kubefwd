//! Add command implementation

use crate::cli::commands::AppContext;
use crate::domain::SkillScope;
use crate::utils::error::Result;

/// Execute the add command
pub async fn execute(
    source: &str,
    name: Option<&str>,
    scope: &str,
    _update_mode: &str,
) -> Result<()> {
    let ctx = AppContext::new()?;

    // Parse scope
    let skill_scope = match scope {
        "global" => SkillScope::Global,
        "local" | _ => {
            let cwd = std::env::current_dir()?;
            SkillScope::Project { path: cwd }
        }
    };

    println!("Adding skill from {}...", source);

    // Add the skill using the service
    use crate::services::SkillService;
    let skill = ctx.skill_service.add(source, name, skill_scope).await?;

    println!();
    println!("✓ Successfully added skill: {}", skill.name);
    println!("  Source: {}", skill.source);
    println!("  Scope: {}", skill.scope);
    println!(
        "  Status: {}",
        if skill.enabled { "enabled" } else { "disabled" }
    );

    Ok(())
}
