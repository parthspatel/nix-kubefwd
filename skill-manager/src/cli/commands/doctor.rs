//! Doctor command implementation

use crate::infra::ConfigManagerImpl;
use crate::utils::error::Result;

/// Issue found during diagnosis
#[derive(Debug)]
struct Issue {
    severity: Severity,
    message: String,
    fix: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Severity {
    Error,
    Warning,
    Info,
}

/// Execute the doctor command
pub async fn execute(fix: bool) -> Result<()> {
    println!("CSM Doctor - Diagnosing issues...");
    println!();

    let mut issues = Vec::new();

    // Check 1: CSM Home exists
    let csm_home = ConfigManagerImpl::detect_csm_home();
    if !csm_home.exists() {
        issues.push(Issue {
            severity: Severity::Error,
            message: format!("CSM home directory does not exist: {}", csm_home.display()),
            fix: Some("Run 'csm init' to initialize".to_string()),
        });
    } else {
        println!("✓ CSM home: {}", csm_home.display());
    }

    // Check 2: Database exists
    let db_path = csm_home.join("registry.db");
    if csm_home.exists() && !db_path.exists() {
        issues.push(Issue {
            severity: Severity::Error,
            message: "Database file missing".to_string(),
            fix: Some("Run 'csm init --force' to reinitialize".to_string()),
        });
    } else if db_path.exists() {
        println!("✓ Database: {}", db_path.display());
    }

    // Check 3: Config file
    let config_path = csm_home.join("config.toml");
    if csm_home.exists() && !config_path.exists() {
        issues.push(Issue {
            severity: Severity::Warning,
            message: "Config file missing (using defaults)".to_string(),
            fix: Some("Run 'csm config reset' to create default config".to_string()),
        });
    } else if config_path.exists() {
        println!("✓ Config: {}", config_path.display());
    }

    // Check 4: Skills directory
    let skills_dir = csm_home.join("skills");
    if csm_home.exists() && !skills_dir.exists() {
        issues.push(Issue {
            severity: Severity::Warning,
            message: "Skills directory missing".to_string(),
            fix: Some("Will be created when first skill is added".to_string()),
        });
    } else if skills_dir.exists() {
        println!("✓ Skills directory: {}", skills_dir.display());
    }

    // Check 5: Legacy migration needed
    if ConfigManagerImpl::needs_migration() {
        if let Some(legacy_path) = ConfigManagerImpl::detect_legacy_home() {
            issues.push(Issue {
                severity: Severity::Warning,
                message: format!(
                    "Legacy data found at {} needs migration",
                    legacy_path.display()
                ),
                fix: Some("Run 'csm migrate' to migrate data".to_string()),
            });
        }
    }

    // If we can connect, check database integrity
    if csm_home.exists() && db_path.exists() {
        match crate::infra::SqliteSkillRepository::new(&db_path) {
            Ok(repo) => {
                println!("✓ Database connection OK");

                // Check skills
                use crate::services::SkillRepository;
                match repo.list().await {
                    Ok(skills) => {
                        println!("✓ Found {} skill(s) in database", skills.len());

                        // Check each skill's content
                        let storage = crate::infra::FileSkillStorage::new(&csm_home);
                        let mut missing_content = 0;

                        for skill in &skills {
                            use crate::services::SkillStorage;
                            if !storage.exists(skill.id).await.unwrap_or(false) {
                                missing_content += 1;
                                issues.push(Issue {
                                    severity: Severity::Error,
                                    message: format!("Missing content for skill: {}", skill.name),
                                    fix: if fix {
                                        Some("Removing orphaned skill record".to_string())
                                    } else {
                                        Some("Run 'csm doctor --fix' to remove".to_string())
                                    },
                                });

                                if fix {
                                    repo.delete(skill.id).await.ok();
                                    println!("  Fixed: Removed orphaned skill '{}'", skill.name);
                                }
                            }
                        }

                        if missing_content == 0 {
                            println!("✓ All skill content files present");
                        }
                    }
                    Err(e) => {
                        issues.push(Issue {
                            severity: Severity::Error,
                            message: format!("Failed to list skills: {}", e),
                            fix: None,
                        });
                    }
                }
            }
            Err(e) => {
                issues.push(Issue {
                    severity: Severity::Error,
                    message: format!("Database error: {}", e),
                    fix: Some("Database may be corrupted. Try 'csm init --force'".to_string()),
                });
            }
        }
    }

    // Print issues summary
    println!();
    if issues.is_empty() {
        println!("✓ No issues found!");
    } else {
        let errors = issues
            .iter()
            .filter(|i| matches!(i.severity, Severity::Error))
            .count();
        let warnings = issues
            .iter()
            .filter(|i| matches!(i.severity, Severity::Warning))
            .count();

        println!("Found {} issue(s):", issues.len());
        println!();

        for issue in &issues {
            let prefix = match issue.severity {
                Severity::Error => "✗ ERROR",
                Severity::Warning => "⚠ WARNING",
                Severity::Info => "ℹ INFO",
            };

            println!("{}: {}", prefix, issue.message);
            if let Some(fix) = &issue.fix {
                println!("  Fix: {}", fix);
            }
        }

        println!();
        if errors > 0 {
            println!("Errors: {}, Warnings: {}", errors, warnings);
            if !fix {
                println!("Run 'csm doctor --fix' to attempt automatic fixes");
            }
        }
    }

    Ok(())
}
