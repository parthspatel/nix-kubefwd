//! Enable and Disable command implementations

use crate::cli::commands::AppContext;
use crate::utils::error::Result;

/// Execute the enable command
pub async fn execute_enable(skill_name: &str) -> Result<()> {
    let ctx = AppContext::new()?;

    use crate::services::SkillService;
    ctx.skill_service.enable(skill_name).await?;

    println!("✓ Enabled skill: {}", skill_name);
    Ok(())
}

/// Execute the disable command
pub async fn execute_disable(skill_name: &str) -> Result<()> {
    let ctx = AppContext::new()?;

    use crate::services::SkillService;
    ctx.skill_service.disable(skill_name).await?;

    println!("✓ Disabled skill: {}", skill_name);
    Ok(())
}
