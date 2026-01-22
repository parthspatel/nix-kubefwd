//! Skill update service implementation

use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{DomainEvent, EventBus, Skill, SkillSource, UpdateMode};
use crate::utils::error::{Error, Result};

use super::traits::{
    GitHubClient, MergeService, SkillRepository, SkillStorage,
    UpdateInfo, UpdateService as UpdateServiceTrait, UrlClient,
};

/// Implementation of the update service
pub struct UpdateServiceImpl<R, S, G, U, M>
where
    R: SkillRepository,
    S: SkillStorage,
    G: GitHubClient,
    U: UrlClient,
    M: MergeService,
{
    repository: Arc<R>,
    storage: Arc<S>,
    github: Arc<G>,
    url_client: Arc<U>,
    merge_service: Arc<M>,
    event_bus: Arc<std::sync::RwLock<EventBus>>,
}

impl<R, S, G, U, M> UpdateServiceImpl<R, S, G, U, M>
where
    R: SkillRepository,
    S: SkillStorage,
    G: GitHubClient,
    U: UrlClient,
    M: MergeService,
{
    /// Create a new update service
    pub fn new(
        repository: Arc<R>,
        storage: Arc<S>,
        github: Arc<G>,
        url_client: Arc<U>,
        merge_service: Arc<M>,
        event_bus: Arc<std::sync::RwLock<EventBus>>,
    ) -> Self {
        Self {
            repository,
            storage,
            github,
            url_client,
            merge_service,
            event_bus,
        }
    }

    /// Publish an event
    fn publish_event(&self, event: DomainEvent) {
        if let Ok(bus) = self.event_bus.read() {
            bus.publish(&event);
        }
    }

    /// Check for updates for a single skill
    async fn check_skill_update(&self, skill: &Skill) -> Result<Option<UpdateInfo>> {
        match &skill.source {
            SkillSource::GitHub {
                owner,
                repo,
                ref_spec,
                commit_sha,
                ..
            } => {
                let current_sha = commit_sha.as_deref().unwrap_or("");
                if current_sha.is_empty() {
                    // No SHA tracked, can't check for updates
                    return Ok(None);
                }

                self.github
                    .check_updates(owner, repo, current_sha, ref_spec.as_deref())
                    .await
            }
            SkillSource::Url { url, etag } => {
                let has_changed = self.url_client.check_modified(url, etag.as_deref()).await?;
                if has_changed {
                    Ok(Some(UpdateInfo {
                        current_sha: etag.clone().unwrap_or_default(),
                        latest_sha: "new".to_string(),
                        commits_behind: 1,
                        commit_messages: vec!["Content changed".to_string()],
                    }))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None), // Local and inline sources don't have updates
        }
    }

    /// Fetch new content for a skill
    async fn fetch_new_content(&self, skill: &Skill) -> Result<(String, Option<String>)> {
        match &skill.source {
            SkillSource::GitHub {
                owner,
                repo,
                path,
                ref_spec,
                ..
            } => {
                let result = self
                    .github
                    .fetch_content(owner, repo, path.as_deref(), ref_spec.as_deref())
                    .await?;
                Ok((result.content, Some(result.commit_sha)))
            }
            SkillSource::Url { url, .. } => {
                let result = self.url_client.fetch(url).await?;
                Ok((result.content, result.etag))
            }
            _ => Err(Error::InvalidSource("Cannot fetch content for this source type".to_string())),
        }
    }
}

#[async_trait]
impl<R, S, G, U, M> UpdateServiceTrait for UpdateServiceImpl<R, S, G, U, M>
where
    R: SkillRepository + 'static,
    S: SkillStorage + 'static,
    G: GitHubClient + 'static,
    U: UrlClient + 'static,
    M: MergeService + 'static,
{
    async fn check(&self) -> Result<Vec<(Skill, UpdateInfo)>> {
        let skills = self.repository.list().await?;
        let mut updates = Vec::new();

        for skill in skills {
            // Skip skills that are not updateable
            if !skill.source.is_updatable() {
                continue;
            }

            // Skip skills set to manual update (they'll be updated explicitly)
            // Note: We still check, just don't auto-apply
            if let Ok(Some(info)) = self.check_skill_update(&skill).await {
                updates.push((skill, info));
            }
        }

        Ok(updates)
    }

    async fn update_skill(&self, name: &str) -> Result<bool> {
        let skill = self
            .repository
            .get_by_name(name)
            .await?
            .ok_or_else(|| Error::SkillNotFound(name.to_string()))?;

        if !skill.source.is_updatable() {
            return Err(Error::InvalidSource(format!(
                "Skill '{}' does not have an updatable source",
                name
            )));
        }

        // Check if update is available
        let update_info = self.check_skill_update(&skill).await?;

        if update_info.is_none() {
            return Ok(false); // No update available
        }

        // Fetch new content
        let (new_content, new_sha) = self.fetch_new_content(&skill).await?;

        // Calculate new hash
        let new_hash = self.storage.hash_content(&new_content);

        // Check if content actually changed
        if new_hash == skill.content_hash {
            return Ok(false); // Content is the same
        }

        let old_hash = skill.content_hash.clone();

        // Store new content
        self.storage.store(skill.id, &new_content).await?;

        // Update skill record
        let mut updated_skill = skill.clone();
        updated_skill.content_hash = new_hash.clone();
        updated_skill.updated_at = chrono::Utc::now();

        // Update source with new SHA if available
        if let Some(sha) = new_sha {
            match &mut updated_skill.source {
                SkillSource::GitHub { commit_sha, .. } => {
                    *commit_sha = Some(sha);
                }
                SkillSource::Url { etag, .. } => {
                    *etag = Some(sha);
                }
                _ => {}
            }
        }

        self.repository.update(&updated_skill).await?;

        // Publish event
        self.publish_event(DomainEvent::skill_updated(
            skill.id,
            &skill.name,
            old_hash,
            new_hash,
        ));

        // Rebuild merged output if skill is enabled
        if skill.enabled {
            self.merge_service.merge(&skill.scope).await?;
        }

        Ok(true)
    }

    async fn update_all(&self) -> Result<Vec<(String, bool)>> {
        let skills = self.repository.list().await?;
        let mut results = Vec::new();

        for skill in skills {
            // Skip non-updatable or manual-update skills
            if !skill.source.is_updatable() {
                continue;
            }

            if skill.update_mode == UpdateMode::Manual {
                continue;
            }

            let name = skill.name.clone();
            match self.update_skill(&name).await {
                Ok(updated) => results.push((name, updated)),
                Err(e) => {
                    tracing::warn!("Failed to update skill {}: {}", name, e);
                    results.push((name, false));
                }
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests will be added in the test planning phase
}
