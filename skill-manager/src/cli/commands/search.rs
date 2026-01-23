//! Search command implementation

use crate::cli::commands::AppContext;
use crate::domain::SkillScope;
use crate::utils::error::Result;

/// Execute the search command
pub async fn execute(query: &str, json: bool) -> Result<()> {
    let ctx = AppContext::new()?;

    use crate::services::SkillService;
    let results = ctx.skill_service.search(query).await?;

    if json {
        let output = serde_json::to_string_pretty(&results)?;
        println!("{}", output);
    } else {
        if results.is_empty() {
            println!("No skills found matching '{}'", query);
            return Ok(());
        }

        println!("Found {} skill(s) matching '{}':", results.len(), query);
        println!();
        println!(
            "{:<20} {:<10} {:<8} {:<30}",
            "NAME", "SCOPE", "STATUS", "SOURCE"
        );
        println!("{}", "-".repeat(70));

        for skill in &results {
            let scope_str = match &skill.scope {
                SkillScope::Global => "global".to_string(),
                SkillScope::Project { path } => {
                    format!(
                        "local:{}",
                        path.file_name().unwrap_or_default().to_string_lossy()
                    )
                }
            };

            let status = if skill.enabled { "enabled" } else { "disabled" };
            let source = skill.source.display_string();

            let source_display = if source.len() > 28 {
                format!("{}...", &source[..25])
            } else {
                source
            };

            println!(
                "{:<20} {:<10} {:<8} {:<30}",
                skill.name, scope_str, status, source_display
            );
        }
    }

    Ok(())
}
