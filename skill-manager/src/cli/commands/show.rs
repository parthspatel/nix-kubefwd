//! Show command implementation

use crate::cli::commands::AppContext;
use crate::domain::SkillScope;
use crate::utils::error::{Error, Result};

/// Execute the show command
pub async fn execute(skill_name: &str, show_content: bool, json: bool) -> Result<()> {
    let ctx = AppContext::new()?;

    // Get the skill
    use crate::services::SkillService;
    let skill = ctx
        .skill_service
        .get(skill_name)
        .await?
        .ok_or_else(|| Error::SkillNotFound(skill_name.to_string()))?;

    if json {
        // JSON output
        let mut output = serde_json::to_value(&skill)?;
        if show_content {
            if let Ok(content) = ctx.skill_service.get_content(skill_name).await {
                output["content"] = serde_json::Value::String(content);
            }
        }
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        // Human-readable output
        println!("Skill: {}", skill.name);
        println!("{}", "=".repeat(40));
        println!();

        if let Some(desc) = &skill.description {
            println!("Description: {}", desc);
            println!();
        }

        println!("ID:          {}", skill.id);
        println!("Source:      {}", skill.source);
        println!(
            "Scope:       {}",
            match &skill.scope {
                SkillScope::Global => "global".to_string(),
                SkillScope::Project { path } => format!("project:{}", path.display()),
            }
        );
        println!(
            "Status:      {}",
            if skill.enabled { "enabled" } else { "disabled" }
        );
        println!("Priority:    {}", skill.priority);
        println!("Update Mode: {}", skill.update_mode);
        println!(
            "Created:     {}",
            skill.created_at.format("%Y-%m-%d %H:%M:%S")
        );
        println!(
            "Updated:     {}",
            skill.updated_at.format("%Y-%m-%d %H:%M:%S")
        );

        if !skill.tags.is_empty() {
            println!("Tags:        {}", skill.tags.join(", "));
        }

        if show_content {
            println!();
            println!("Content:");
            println!("{}", "-".repeat(40));
            match ctx.skill_service.get_content(skill_name).await {
                Ok(content) => println!("{}", content),
                Err(e) => println!("(Error reading content: {})", e),
            }
        }
    }

    Ok(())
}
