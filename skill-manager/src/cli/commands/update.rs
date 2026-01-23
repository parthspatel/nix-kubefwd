//! Update command implementation

use crate::cli::commands::AppContext;
use crate::utils::error::Result;

/// Execute the update command
pub async fn execute(skill_name: Option<&str>, check_only: bool, dry_run: bool) -> Result<()> {
    let ctx = AppContext::new()?;

    use crate::services::UpdateService;

    if let Some(name) = skill_name {
        // Update specific skill
        if check_only || dry_run {
            println!("Checking for updates to '{}'...", name);
            let updates = ctx.update_service.check().await?;
            let update = updates.iter().find(|(s, _)| s.name == name);

            match update {
                Some((skill, info)) => {
                    println!("Update available for '{}':", skill.name);
                    println!(
                        "  Current: {}",
                        info.current_sha.chars().take(7).collect::<String>()
                    );
                    println!(
                        "  Latest:  {}",
                        info.latest_sha.chars().take(7).collect::<String>()
                    );
                    println!("  Commits behind: {}", info.commits_behind);

                    if !info.commit_messages.is_empty() {
                        println!("  Changes:");
                        for msg in info.commit_messages.iter().take(5) {
                            println!("    - {}", msg);
                        }
                    }

                    if dry_run {
                        println!();
                        println!("(dry-run) Would update this skill");
                    }
                }
                None => {
                    println!("'{}' is up to date", name);
                }
            }
        } else {
            println!("Updating '{}'...", name);
            let updated = ctx.update_service.update_skill(name).await?;
            if updated {
                println!("✓ Updated '{}'", name);
            } else {
                println!("'{}' is already up to date", name);
            }
        }
    } else {
        // Check/update all skills
        if check_only {
            println!("Checking for updates...");
            let updates = ctx.update_service.check().await?;

            if updates.is_empty() {
                println!("All skills are up to date");
            } else {
                println!("Updates available for {} skill(s):", updates.len());
                println!();

                for (skill, info) in &updates {
                    println!("  {} ({} commits behind)", skill.name, info.commits_behind);
                }

                println!();
                println!("Run 'csm update' to apply all updates");
            }
        } else if dry_run {
            println!("Checking for updates (dry-run)...");
            let updates = ctx.update_service.check().await?;

            if updates.is_empty() {
                println!("All skills are up to date");
            } else {
                println!("Would update {} skill(s):", updates.len());
                for (skill, info) in &updates {
                    println!("  {} ({} commits behind)", skill.name, info.commits_behind);
                }
            }
        } else {
            println!("Updating all skills...");
            let results = ctx.update_service.update_all().await?;

            let updated: Vec<_> = results.iter().filter(|(_, success)| *success).collect();
            let failed: Vec<_> = results.iter().filter(|(_, success)| !*success).collect();

            if updated.is_empty() && failed.is_empty() {
                println!("All skills are up to date");
            } else {
                if !updated.is_empty() {
                    println!("✓ Updated {} skill(s):", updated.len());
                    for (name, _) in &updated {
                        println!("    {}", name);
                    }
                }

                if !failed.is_empty() {
                    println!("✗ Failed to update {} skill(s):", failed.len());
                    for (name, _) in &failed {
                        println!("    {}", name);
                    }
                }
            }
        }
    }

    Ok(())
}
