//! Create command implementation

use crate::cli::commands::AppContext;
use crate::domain::{Skill, SkillScope, SkillSource};
use crate::utils::error::{Error, Result};

/// Execute the create command
pub async fn execute(name: &str, from: Option<&str>, scope: &str, edit: bool) -> Result<()> {
    let ctx = AppContext::new()?;

    // Parse scope
    let skill_scope = match scope {
        "global" => SkillScope::Global,
        "local" | _ => {
            let cwd = std::env::current_dir()?;
            SkillScope::Project { path: cwd }
        }
    };

    // Check if skill already exists
    use crate::services::SkillRepository;
    if ctx.skill_repo.exists(name).await? {
        return Err(Error::SkillExists(name.to_string()));
    }

    // Get initial content
    let content = if let Some(from_path) = from {
        // Read from existing file
        tokio::fs::read_to_string(from_path)
            .await
            .map_err(|_| Error::FileNotFound(from_path.into()))?
    } else {
        // Create template content
        format!(
            r#"# {}

## Overview

(Describe this skill)

## Instructions

- (Add your instructions here)
"#,
            name
        )
    };

    // Create the skill
    let skill = Skill::builder(name)
        .source(SkillSource::Inline)
        .scope(skill_scope.clone())
        .build();

    // Store content
    use crate::services::SkillStorage;
    let hash = ctx.storage.store(skill.id, &content).await?;

    // Save skill with updated hash
    let mut skill = skill;
    skill.content_hash = hash;
    ctx.skill_repo.create(&skill).await?;

    // Rebuild merged output
    use crate::services::MergeService;
    ctx.merge_service.merge(&skill_scope).await?;

    println!("✓ Created skill: {}", name);
    println!("  Scope: {}", skill_scope);

    // Open in editor if requested
    if edit {
        let skill_path = ctx.storage.get_path(skill.id);
        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| "vi".to_string());

        println!("Opening in {}...", editor);

        let status = std::process::Command::new(&editor)
            .arg(&skill_path)
            .status()?;

        if status.success() {
            // Re-read content and update hash
            let new_content = tokio::fs::read_to_string(&skill_path).await?;
            let new_hash = ctx.storage.store(skill.id, &new_content).await?;

            let mut updated_skill = skill;
            updated_skill.content_hash = new_hash;
            updated_skill.updated_at = chrono::Utc::now();
            ctx.skill_repo.update(&updated_skill).await?;

            // Rebuild merged output
            ctx.merge_service.merge(&skill_scope).await?;

            println!("✓ Skill content updated");
        }
    }

    Ok(())
}
