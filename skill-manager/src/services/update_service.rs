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
    use crate::domain::{SkillScope, SkillSource, Skill, EventBus, UpdateMode};
    use crate::services::traits::mocks::*;
    use crate::services::traits::{MergeService as MergeServiceTrait, UpdateInfo};
    use crate::utils::error::Result;
    use async_trait::async_trait;
    use std::sync::{Arc, RwLock};

    // Mock merge service for tests
    struct MockMergeService;

    #[async_trait]
    impl MergeServiceTrait for MockMergeService {
        async fn merge(&self, _scope: &SkillScope) -> Result<String> {
            Ok("merged".to_string())
        }
        async fn rebuild_all(&self) -> Result<()> {
            Ok(())
        }
    }

    fn create_github_skill(name: &str, sha: &str) -> Skill {
        Skill::builder(name)
            .source(SkillSource::GitHub {
                owner: "owner".to_string(),
                repo: "repo".to_string(),
                path: None,
                ref_spec: Some("main".to_string()),
                commit_sha: Some(sha.to_string()),
            })
            .build()
    }

    fn create_url_skill(name: &str) -> Skill {
        Skill::builder(name)
            .source(SkillSource::Url {
                url: "https://example.com/skill.md".to_string(),
                etag: Some("etag123".to_string()),
            })
            .build()
    }

    // S-UP-01: test_check_update_github_new_commit (mock will return update info)
    #[tokio::test]
    async fn test_check_update_github_new_commit() {
        use crate::services::UpdateService;
        let repo = MockSkillRepository::new();
        let storage = MockSkillStorage::new();
        let github = MockGitHubClient::new();

        // Set up update info
        *github.update_info.lock().unwrap() = Some(UpdateInfo {
            current_sha: "old_sha".to_string(),
            latest_sha: "new_sha".to_string(),
            commits_behind: 2,
            commit_messages: vec!["commit 1".to_string(), "commit 2".to_string()],
        });

        let skill = create_github_skill("test-skill", "old_sha");
        repo.skills.lock().unwrap().push(skill.clone());
        storage.content.lock().unwrap().insert(skill.id, "content".to_string());

        let service = super::UpdateServiceImpl::new(
            Arc::new(repo),
            Arc::new(storage),
            Arc::new(github),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let updates = service.check().await.unwrap();
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].1.commits_behind, 2);
    }

    // S-UP-02: test_check_update_github_no_change
    #[tokio::test]
    async fn test_check_update_github_no_change() {
        use crate::services::UpdateService;
        let repo = MockSkillRepository::new();
        let github = MockGitHubClient::new();
        // update_info is None by default, meaning no update

        let skill = create_github_skill("test-skill", "current_sha");
        repo.skills.lock().unwrap().push(skill);

        let service = super::UpdateServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(github),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let updates = service.check().await.unwrap();
        assert!(updates.is_empty());
    }

    // S-UP-03: test_check_update_url_etag_changed
    #[tokio::test]
    async fn test_check_update_url_etag_changed() {
        use crate::services::UpdateService;
        let repo = MockSkillRepository::new();
        let url_client = MockUrlClient::new();
        *url_client.modified.lock().unwrap() = true; // URL has changed

        let skill = create_url_skill("test-skill");
        repo.skills.lock().unwrap().push(skill);

        let service = super::UpdateServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(url_client),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let updates = service.check().await.unwrap();
        assert_eq!(updates.len(), 1);
    }

    // S-UP-05: test_apply_update_success
    #[tokio::test]
    async fn test_apply_update_success() {
        use crate::services::UpdateService;
        let repo = MockSkillRepository::new();
        let storage = MockSkillStorage::new();
        let github = MockGitHubClient::with_content(
            "# Updated Content".to_string(),
            "new_file_sha".to_string(),
            "new_commit_sha".to_string(),
        );

        // Set up update info
        *github.update_info.lock().unwrap() = Some(UpdateInfo {
            current_sha: "old_sha".to_string(),
            latest_sha: "new_sha".to_string(),
            commits_behind: 1,
            commit_messages: vec!["update".to_string()],
        });

        let mut skill = create_github_skill("test-skill", "old_sha");
        skill.content_hash = "old_hash".to_string();
        repo.skills.lock().unwrap().push(skill.clone());
        storage.content.lock().unwrap().insert(skill.id, "# Old Content".to_string());

        let service = super::UpdateServiceImpl::new(
            Arc::new(repo),
            Arc::new(storage),
            Arc::new(github),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let updated = service.update_skill("test-skill").await.unwrap();
        assert!(updated);
    }

    // S-UP-07: test_update_mode_manual_skipped
    #[tokio::test]
    async fn test_update_mode_manual_skipped() {
        use crate::services::UpdateService;
        let repo = MockSkillRepository::new();
        let github = MockGitHubClient::new();

        // Set up update info
        *github.update_info.lock().unwrap() = Some(UpdateInfo {
            current_sha: "old_sha".to_string(),
            latest_sha: "new_sha".to_string(),
            commits_behind: 1,
            commit_messages: vec![],
        });

        let mut skill = create_github_skill("manual-skill", "old_sha");
        skill.update_mode = UpdateMode::Manual;
        repo.skills.lock().unwrap().push(skill);

        let service = super::UpdateServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(github),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        // update_all should skip manual skills
        let results = service.update_all().await.unwrap();
        assert!(results.is_empty()); // Manual skill was skipped
    }

    #[tokio::test]
    async fn test_update_skill_not_found() {
        use crate::services::UpdateService;
        let service = super::UpdateServiceImpl::new(
            Arc::new(MockSkillRepository::new()),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let result = service.update_skill("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_inline_skill_error() {
        use crate::services::UpdateService;
        let repo = MockSkillRepository::new();
        repo.skills.lock().unwrap().push(Skill::new("inline", SkillSource::Inline, SkillScope::Global));

        let service = super::UpdateServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let result = service.update_skill("inline").await;
        assert!(result.is_err()); // Cannot update inline source
    }
}
