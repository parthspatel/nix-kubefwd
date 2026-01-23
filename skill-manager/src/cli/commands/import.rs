//! Import command implementation

use crate::cli::commands::AppContext;
use crate::domain::{Skill, SkillScope, SkillSource};
use crate::utils::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// Exported skill data (matches export format)
#[derive(Debug, Serialize, Deserialize)]
struct ExportedSkill {
    name: String,
    description: Option<String>,
    source: String,
    scope: String,
    enabled: bool,
    priority: i32,
    #[serde(default)]
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

/// Execute the import command
pub async fn execute(source: &str, merge: bool, dry_run: bool) -> Result<()> {
    let ctx = AppContext::new()?;

    // Read import file
    let content = if source.starts_with("http://") || source.starts_with("https://") {
        // Fetch from URL
        let client = reqwest::Client::new();
        client.get(source).send().await?.text().await?
    } else {
        // Read from file
        tokio::fs::read_to_string(source)
            .await
            .map_err(|_| Error::FileNotFound(source.into()))?
    };

    // Parse import data
    let import_data: ExportData = if source.ends_with(".toml") {
        toml::from_str(&content)?
    } else {
        serde_json::from_str(&content)?
    };

    println!(
        "Importing {} skill(s) from export (version {})",
        import_data.skills.len(),
        import_data.version
    );

    use crate::services::{SkillRepository, SkillStorage};

    let mut imported = 0;
    let mut skipped = 0;
    let mut errors = 0;

    for exported in &import_data.skills {
        // Check if skill already exists
        let exists = ctx.skill_repo.exists(&exported.name).await?;

        if exists && !merge {
            println!("  Skipping '{}' (already exists)", exported.name);
            skipped += 1;
            continue;
        }

        if dry_run {
            if exists {
                println!("  Would update: {}", exported.name);
            } else {
                println!("  Would import: {}", exported.name);
            }
            imported += 1;
            continue;
        }

        // Parse scope
        let scope = if exported.scope.starts_with("project:") {
            let path = exported.scope.strip_prefix("project:").unwrap_or(".");
            SkillScope::Project {
                path: std::path::PathBuf::from(path),
            }
        } else {
            SkillScope::Global
        };

        // Create or update skill
        if exists && merge {
            // Update existing skill
            if let Some(mut skill) = ctx.skill_repo.get_by_name(&exported.name).await? {
                skill.description = exported.description.clone();
                skill.enabled = exported.enabled;
                skill.priority = exported.priority;
                skill.tags = exported.tags.clone();
                skill.updated_at = chrono::Utc::now();

                // Update content
                let hash = ctx.storage.store(skill.id, &exported.content).await?;
                skill.content_hash = hash;

                ctx.skill_repo.update(&skill).await?;
                println!("  Updated: {}", exported.name);
                imported += 1;
            }
        } else {
            // Create new skill
            let mut skill = Skill::builder(&exported.name)
                .source(SkillSource::Inline)
                .scope(scope)
                .enabled(exported.enabled)
                .priority(exported.priority)
                .tags(exported.tags.clone())
                .build();

            if let Some(desc) = &exported.description {
                skill.description = Some(desc.clone());
            }

            // Store content
            match ctx.storage.store(skill.id, &exported.content).await {
                Ok(hash) => {
                    skill.content_hash = hash;
                    match ctx.skill_repo.create(&skill).await {
                        Ok(_) => {
                            println!("  Imported: {}", exported.name);
                            imported += 1;
                        }
                        Err(e) => {
                            println!("  Error importing '{}': {}", exported.name, e);
                            errors += 1;
                        }
                    }
                }
                Err(e) => {
                    println!("  Error storing '{}': {}", exported.name, e);
                    errors += 1;
                }
            }
        }
    }

    println!();
    if dry_run {
        println!("(dry-run) Would import {} skill(s)", imported);
    } else {
        println!(
            "✓ Imported: {}, Skipped: {}, Errors: {}",
            imported, skipped, errors
        );

        // Rebuild merged files
        if imported > 0 {
            use crate::services::MergeService;
            ctx.merge_service.rebuild_all().await?;
        }
    }

    Ok(())
}
