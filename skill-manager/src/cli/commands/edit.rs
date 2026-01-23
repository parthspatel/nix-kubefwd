//! Edit command implementation

use crate::cli::commands::AppContext;
use crate::utils::error::{Error, Result};

/// Execute the edit command
pub async fn execute(skill_name: &str, editor: Option<&str>) -> Result<()> {
    let ctx = AppContext::new()?;

    // Get the skill
    use crate::services::SkillRepository;
    let skill = ctx
        .skill_repo
        .get_by_name(skill_name)
        .await?
        .ok_or_else(|| Error::SkillNotFound(skill_name.to_string()))?;

    // Get path to skill content
    use crate::services::SkillStorage;
    let skill_path = ctx.storage.get_path(skill.id);

    if !skill_path.exists() {
        return Err(Error::FileNotFound(skill_path));
    }

    // Determine editor
    let editor_cmd = editor
        .map(String::from)
        .or_else(|| std::env::var("EDITOR").ok())
        .or_else(|| std::env::var("VISUAL").ok())
        .unwrap_or_else(|| "vi".to_string());

    println!("Opening '{}' in {}...", skill_name, editor_cmd);

    // Get content hash before edit
    let content_before = tokio::fs::read_to_string(&skill_path).await?;
    let hash_before = ctx.storage.hash_content(&content_before);

    // Open editor
    let status = std::process::Command::new(&editor_cmd)
        .arg(&skill_path)
        .status()?;

    if !status.success() {
        return Err(Error::Other(format!(
            "Editor exited with status: {}",
            status
        )));
    }

    // Check if content changed
    let content_after = tokio::fs::read_to_string(&skill_path).await?;
    let hash_after = ctx.storage.hash_content(&content_after);

    if hash_before == hash_after {
        println!("No changes made");
        return Ok(());
    }

    // Update skill metadata
    let mut updated_skill = skill.clone();
    updated_skill.content_hash = hash_after;
    updated_skill.updated_at = chrono::Utc::now();
    ctx.skill_repo.update(&updated_skill).await?;

    // Rebuild merged output
    use crate::services::MergeService;
    ctx.merge_service.merge(&skill.scope).await?;

    println!("✓ Skill '{}' updated", skill_name);

    Ok(())
}
