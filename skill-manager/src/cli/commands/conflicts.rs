//! Conflicts command implementation

use crate::cli::commands::AppContext;
use crate::utils::error::Result;

/// Execute the conflicts command
pub async fn execute(resolve: bool, json: bool) -> Result<()> {
    let ctx = AppContext::new()?;

    use crate::services::ConflictService;

    // Detect conflicts
    println!("Detecting conflicts...");
    let conflicts = ctx.conflict_service.detect().await?;

    if conflicts.is_empty() {
        println!("✓ No conflicts detected");
        return Ok(());
    }

    if json {
        let output = serde_json::to_string_pretty(&conflicts)?;
        println!("{}", output);
        return Ok(());
    }

    println!("Found {} conflict(s):", conflicts.len());
    println!();

    use crate::services::SkillRepository;

    for (i, conflict) in conflicts.iter().enumerate() {
        let skill_a = ctx.skill_repo.get(conflict.skill_a_id).await?;
        let skill_b = ctx.skill_repo.get(conflict.skill_b_id).await?;

        let name_a = skill_a
            .map(|s| s.name)
            .unwrap_or_else(|| "unknown".to_string());
        let name_b = skill_b
            .map(|s| s.name)
            .unwrap_or_else(|| "unknown".to_string());

        println!("{}. {} <-> {}", i + 1, name_a, name_b);
        println!("   Type: {:?}", conflict.conflict_type);
        println!("   Description: {}", conflict.description);

        if let Some(ref suggestion) = conflict.suggestion {
            println!("   Suggestion: {}", suggestion);
        }

        if let (Some(line_a), Some(line_b)) = (conflict.line_a, conflict.line_b) {
            println!("   Lines: {} (skill A) vs {} (skill B)", line_a, line_b);
        }

        println!("   Status: {}", conflict.status);
        println!();
    }

    if resolve {
        println!("Interactive resolution:");
        println!();

        use crate::domain::ResolutionStrategy;
        use std::io::{self, Write};

        for conflict in &conflicts {
            let skill_a = ctx.skill_repo.get(conflict.skill_a_id).await?;
            let skill_b = ctx.skill_repo.get(conflict.skill_b_id).await?;

            let name_a = skill_a
                .map(|s| s.name)
                .unwrap_or_else(|| "unknown".to_string());
            let name_b = skill_b
                .map(|s| s.name)
                .unwrap_or_else(|| "unknown".to_string());

            println!("Conflict: {} <-> {}", name_a, name_b);
            println!("  {}", conflict.description);
            println!();
            println!("Options:");
            println!("  1. Keep '{}' (disable '{}')", name_a, name_b);
            println!("  2. Keep '{}' (disable '{}')", name_b, name_a);
            println!("  3. Ignore this conflict");
            println!("  4. Skip (decide later)");
            println!();

            print!("Choose [1-4]: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
                "1" => {
                    ctx.conflict_service
                        .resolve(conflict.id, ResolutionStrategy::DisableSkillB)
                        .await?;
                    println!("✓ Keeping '{}', disabled '{}'", name_a, name_b);
                }
                "2" => {
                    ctx.conflict_service
                        .resolve(conflict.id, ResolutionStrategy::DisableSkillA)
                        .await?;
                    println!("✓ Keeping '{}', disabled '{}'", name_b, name_a);
                }
                "3" => {
                    ctx.conflict_service.ignore(conflict.id).await?;
                    println!("✓ Ignored conflict");
                }
                _ => {
                    println!("Skipped");
                }
            }

            println!();
        }
    }

    Ok(())
}
