//! Export command implementation

use crate::cli::commands::AppContext;
use crate::utils::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// Exported skill data
#[derive(Debug, Serialize, Deserialize)]
struct ExportedSkill {
    name: String,
    description: Option<String>,
    source: String,
    scope: String,
    enabled: bool,
    priority: i32,
    tags: Vec<String>,
    content: String,
}

/// Export data structure
#[derive(Debug, Serialize, Deserialize)]
struct ExportData {
    version: String,
    exported_at: String,
    skills: Vec<ExportedSkill>,
}

/// Execute the export command
pub async fn execute(
    all: bool,
    skill_name: Option<&str>,
    format: &str,
    output: Option<&str>,
) -> Result<()> {
    let ctx = AppContext::new()?;

    use crate::services::{SkillRepository, SkillStorage};

    // Get skills to export
    let skills = if let Some(name) = skill_name {
        let skill = ctx
            .skill_repo
            .get_by_name(name)
            .await?
            .ok_or_else(|| Error::SkillNotFound(name.to_string()))?;
        vec![skill]
    } else if all {
        ctx.skill_repo.list().await?
    } else {
        // Default to enabled skills
        ctx.skill_repo.list_enabled().await?
    };

    if skills.is_empty() {
        println!("No skills to export");
        return Ok(());
    }

    // Build export data
    let mut exported_skills = Vec::new();
    for skill in &skills {
        let content = ctx.storage.read(skill.id).await.unwrap_or_default();
        exported_skills.push(ExportedSkill {
            name: skill.name.clone(),
            description: skill.description.clone(),
            source: skill.source.display_string(),
            scope: format!("{}", skill.scope),
            enabled: skill.enabled,
            priority: skill.priority,
            tags: skill.tags.clone(),
            content,
        });
    }

    let export_data = ExportData {
        version: "1.0".to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        skills: exported_skills,
    };

    // Serialize
    let output_str = match format {
        "toml" => toml::to_string_pretty(&export_data)?,
        "json" | _ => serde_json::to_string_pretty(&export_data)?,
    };

    // Write output
    if let Some(path) = output {
        tokio::fs::write(path, &output_str).await?;
        println!("✓ Exported {} skill(s) to {}", skills.len(), path);
    } else {
        println!("{}", output_str);
    }

    Ok(())
}
