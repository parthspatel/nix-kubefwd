//! Sync command implementation

use crate::cli::commands::AppContext;
use crate::utils::error::Result;

/// Execute the sync command
pub async fn execute(rebuild: bool, verify: bool) -> Result<()> {
    let ctx = AppContext::new()?;

    if rebuild {
        println!("Rebuilding merged CLAUDE.md files...");
        use crate::services::MergeService;
        ctx.merge_service.rebuild_all().await?;
        println!("✓ Rebuilt all merged files");
    }

    if verify {
        println!("Verifying skill integrity...");

        use crate::services::SkillRepository;
        let skills = ctx.skill_repo.list().await?;
        let mut issues = Vec::new();

        for skill in &skills {
            // Check if content exists
            use crate::services::SkillStorage;
            if !ctx.storage.exists(skill.id).await? {
                issues.push(format!("Missing content for skill: {}", skill.name));
            }
        }

        if issues.is_empty() {
            println!("✓ All skills verified successfully");
        } else {
            println!("Found {} issue(s):", issues.len());
            for issue in &issues {
                println!("  - {}", issue);
            }
        }
    }

    if !rebuild && !verify {
        // Default behavior: just rebuild
        println!("Syncing skill state...");
        use crate::services::MergeService;
        ctx.merge_service.rebuild_all().await?;
        println!("✓ Sync complete");
    }

    Ok(())
}
