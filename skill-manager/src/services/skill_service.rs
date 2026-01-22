//! Skill management service implementation

use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{parse_source, DomainEvent, EventBus, Skill, SkillScope, SkillSource};
use crate::utils::error::{Error, Result};

use super::traits::{
    GitHubClient, MergeService, SkillRepository, SkillService as SkillServiceTrait, SkillStorage,
    UrlClient,
};

/// Implementation of the skill management service
pub struct SkillServiceImpl<R, S, G, U, M>
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

impl<R, S, G, U, M> SkillServiceImpl<R, S, G, U, M>
where
    R: SkillRepository,
    S: SkillStorage,
    G: GitHubClient,
    U: UrlClient,
    M: MergeService,
{
    /// Create a new skill service
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

    /// Fetch content from a source
    async fn fetch_content(&self, source: &SkillSource) -> Result<String> {
        match source {
            SkillSource::Local { path } => {
                tokio::fs::read_to_string(path)
                    .await
                    .map_err(|e| Error::FileNotFound(path.clone()))
            }
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
                Ok(result.content)
            }
            SkillSource::Url { url, .. } => {
                let result = self.url_client.fetch(url).await?;
                Ok(result.content)
            }
            SkillSource::Inline => Err(Error::InvalidSource(
                "Cannot fetch content for inline source".to_string(),
            )),
        }
    }

    /// Validate skill content
    fn validate_content(&self, content: &str) -> Result<()> {
        if content.is_empty() {
            return Err(Error::InvalidContent("Content cannot be empty".to_string()));
        }

        if !content.is_ascii() && content.chars().any(|c| c == '\0') {
            return Err(Error::InvalidContent(
                "Content contains invalid characters".to_string(),
            ));
        }

        // Check for binary content
        let non_text_chars = content
            .chars()
            .filter(|c| !c.is_ascii_graphic() && !c.is_ascii_whitespace())
            .count();

        if non_text_chars > content.len() / 10 {
            return Err(Error::InvalidContent(
                "Content appears to be binary".to_string(),
            ));
        }

        Ok(())
    }

    /// Publish an event
    fn publish_event(&self, event: DomainEvent) {
        if let Ok(bus) = self.event_bus.read() {
            bus.publish(&event);
        }
    }
}

#[async_trait]
impl<R, S, G, U, M> SkillServiceTrait for SkillServiceImpl<R, S, G, U, M>
where
    R: SkillRepository + 'static,
    S: SkillStorage + 'static,
    G: GitHubClient + 'static,
    U: UrlClient + 'static,
    M: MergeService + 'static,
{
    async fn add(&self, source_str: &str, name: Option<&str>, scope: SkillScope) -> Result<Skill> {
        // Parse the source
        let parsed = parse_source(source_str)
            .map_err(|e| Error::InvalidSource(e.to_string()))?;

        // Determine the name
        let skill_name = name
            .map(String::from)
            .unwrap_or(parsed.suggested_name);

        // Validate name
        if skill_name.is_empty() {
            return Err(Error::InvalidSkillName("Name cannot be empty".to_string()));
        }

        // Check if skill already exists
        if self.repository.exists(&skill_name).await? {
            return Err(Error::SkillExists(skill_name));
        }

        // Fetch content
        let content = self.fetch_content(&parsed.source).await?;

        // Validate content
        self.validate_content(&content)?;

        // Create skill
        let mut skill = Skill::builder(&skill_name)
            .source(parsed.source.clone())
            .scope(scope.clone())
            .build();

        // Store content and get hash
        let hash = self.storage.store(skill.id, &content).await?;
        skill.content_hash = hash;

        // Save to repository
        self.repository.create(&skill).await?;

        // Publish event
        self.publish_event(DomainEvent::skill_added(
            skill.id,
            &skill.name,
            skill.source.clone(),
            skill.scope.clone(),
        ));

        // Rebuild merged output
        self.merge_service.merge(&scope).await?;

        Ok(skill)
    }

    async fn remove(&self, name: &str) -> Result<()> {
        // Get the skill
        let skill = self
            .repository
            .get_by_name(name)
            .await?
            .ok_or_else(|| Error::SkillNotFound(name.to_string()))?;

        let scope = skill.scope.clone();

        // Delete content
        self.storage.delete(skill.id).await?;

        // Delete from repository
        self.repository.delete(skill.id).await?;

        // Publish event
        self.publish_event(DomainEvent::skill_removed(skill.id, name));

        // Rebuild merged output
        self.merge_service.merge(&scope).await?;

        Ok(())
    }

    async fn enable(&self, name: &str) -> Result<()> {
        let mut skill = self
            .repository
            .get_by_name(name)
            .await?
            .ok_or_else(|| Error::SkillNotFound(name.to_string()))?;

        if skill.enabled {
            return Ok(()); // Already enabled
        }

        skill.enabled = true;
        skill.updated_at = chrono::Utc::now();

        self.repository.update(&skill).await?;

        self.publish_event(DomainEvent::skill_enabled(skill.id, name));

        // Rebuild merged output
        self.merge_service.merge(&skill.scope).await?;

        Ok(())
    }

    async fn disable(&self, name: &str) -> Result<()> {
        let mut skill = self
            .repository
            .get_by_name(name)
            .await?
            .ok_or_else(|| Error::SkillNotFound(name.to_string()))?;

        if !skill.enabled {
            return Ok(()); // Already disabled
        }

        skill.enabled = false;
        skill.updated_at = chrono::Utc::now();

        self.repository.update(&skill).await?;

        self.publish_event(DomainEvent::skill_disabled(skill.id, name));

        // Rebuild merged output
        self.merge_service.merge(&skill.scope).await?;

        Ok(())
    }

    async fn get(&self, name: &str) -> Result<Option<Skill>> {
        self.repository.get_by_name(name).await
    }

    async fn list(&self, scope: Option<SkillScope>, enabled_only: bool) -> Result<Vec<Skill>> {
        let skills = match scope {
            Some(s) => self.repository.list_by_scope(&s).await?,
            None => self.repository.list().await?,
        };

        if enabled_only {
            Ok(skills.into_iter().filter(|s| s.enabled).collect())
        } else {
            Ok(skills)
        }
    }

    async fn search(&self, query: &str) -> Result<Vec<Skill>> {
        self.repository.search(query).await
    }

    async fn get_content(&self, name: &str) -> Result<String> {
        let skill = self
            .repository
            .get_by_name(name)
            .await?
            .ok_or_else(|| Error::SkillNotFound(name.to_string()))?;

        self.storage.read(skill.id).await
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{SkillScope, SkillSource, Skill, EventBus};
    use crate::services::traits::mocks::*;
    use crate::services::traits::MergeService as MergeServiceTrait;
    use crate::utils::error::{Error, Result};
    use async_trait::async_trait;
    use std::sync::{Arc, RwLock};

    // Helper to create a mock merge service
    struct MockMergeService;

    #[async_trait]
    impl MergeServiceTrait for MockMergeService {
        async fn merge(&self, _scope: &SkillScope) -> Result<String> {
            Ok("merged content".to_string())
        }
        async fn rebuild_all(&self) -> Result<()> {
            Ok(())
        }
    }

    fn create_test_service() -> super::SkillServiceImpl<MockSkillRepository, MockSkillStorage, MockGitHubClient, MockUrlClient, MockMergeService> {
        super::SkillServiceImpl::new(
            Arc::new(MockSkillRepository::new()),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::with_content(
                "# Test Skill\n\nTest content".to_string(),
                "abc123".to_string(),
                "def456".to_string(),
            )),
            Arc::new(MockUrlClient::with_content("# URL Skill\n\nURL content".to_string())),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        )
    }

    // S-SK-01: test_add_skill_from_github
    #[tokio::test]
    async fn test_add_skill_from_github() {
        use crate::services::SkillService;
        let service = create_test_service();

        let result = service.add("github:owner/repo", None, SkillScope::Global).await;
        assert!(result.is_ok());

        let skill = result.unwrap();
        assert_eq!(skill.name, "repo");
        assert!(matches!(skill.source, SkillSource::GitHub { .. }));
        assert_eq!(skill.scope, SkillScope::Global);
        assert!(skill.enabled);
    }

    // S-SK-03: test_add_skill_from_url
    #[tokio::test]
    async fn test_add_skill_from_url() {
        use crate::services::SkillService;
        let service = create_test_service();

        let result = service.add("https://example.com/skill.md", None, SkillScope::Global).await;
        assert!(result.is_ok());

        let skill = result.unwrap();
        assert_eq!(skill.name, "skill");
        assert!(matches!(skill.source, SkillSource::Url { .. }));
    }

    // S-SK-04: test_add_skill_duplicate_error
    #[tokio::test]
    async fn test_add_skill_duplicate_error() {
        use crate::services::SkillService;
        let repo = MockSkillRepository::new();
        let existing_skill = Skill::new("my-skill", SkillSource::Inline, SkillScope::Global);
        repo.skills.lock().unwrap().push(existing_skill);

        let service = super::SkillServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::with_content("content".to_string(), "sha".to_string(), "sha".to_string())),
            Arc::new(MockUrlClient::with_content("content".to_string())),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let result = service.add("github:owner/my-skill", None, SkillScope::Global).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::SkillExists(_)));
    }

    // S-SK-05: test_remove_skill_success
    #[tokio::test]
    async fn test_remove_skill_success() {
        use crate::services::SkillService;
        let repo = MockSkillRepository::new();
        let storage = MockSkillStorage::new();
        let skill = Skill::new("test-skill", SkillSource::Inline, SkillScope::Global);
        repo.skills.lock().unwrap().push(skill.clone());
        storage.content.lock().unwrap().insert(skill.id, "content".to_string());

        let service = super::SkillServiceImpl::new(
            Arc::new(repo),
            Arc::new(storage),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let result = service.remove("test-skill").await;
        assert!(result.is_ok());

        // Verify skill was removed
        let found = service.get("test-skill").await.unwrap();
        assert!(found.is_none());
    }

    // S-SK-06: test_remove_skill_not_found
    #[tokio::test]
    async fn test_remove_skill_not_found() {
        use crate::services::SkillService;
        let service = create_test_service();

        let result = service.remove("nonexistent").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::SkillNotFound(_)));
    }

    // S-SK-07: test_enable_skill
    #[tokio::test]
    async fn test_enable_skill() {
        use crate::services::SkillService;
        let repo = MockSkillRepository::new();
        let mut skill = Skill::new("test-skill", SkillSource::Inline, SkillScope::Global);
        skill.enabled = false;
        repo.skills.lock().unwrap().push(skill);

        let service = super::SkillServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let result = service.enable("test-skill").await;
        assert!(result.is_ok());

        let skill = service.get("test-skill").await.unwrap().unwrap();
        assert!(skill.enabled);
    }

    // S-SK-08: test_disable_skill
    #[tokio::test]
    async fn test_disable_skill() {
        use crate::services::SkillService;
        let repo = MockSkillRepository::new();
        let skill = Skill::new("test-skill", SkillSource::Inline, SkillScope::Global);
        repo.skills.lock().unwrap().push(skill);

        let service = super::SkillServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let result = service.disable("test-skill").await;
        assert!(result.is_ok());

        let skill = service.get("test-skill").await.unwrap().unwrap();
        assert!(!skill.enabled);
    }

    // S-SK-09: test_get_skill_by_name
    #[tokio::test]
    async fn test_get_skill_by_name() {
        use crate::services::SkillService;
        let repo = MockSkillRepository::new();
        let skill = Skill::new("test-skill", SkillSource::Inline, SkillScope::Global);
        repo.skills.lock().unwrap().push(skill.clone());

        let service = super::SkillServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let result = service.get("test-skill").await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "test-skill");

        let result = service.get("nonexistent").await.unwrap();
        assert!(result.is_none());
    }

    // S-SK-10: test_list_skills_all
    #[tokio::test]
    async fn test_list_skills_all() {
        use crate::services::SkillService;
        let repo = MockSkillRepository::new();
        repo.skills.lock().unwrap().push(Skill::new("skill-1", SkillSource::Inline, SkillScope::Global));
        repo.skills.lock().unwrap().push(Skill::new("skill-2", SkillSource::Inline, SkillScope::Global));

        let service = super::SkillServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let skills = service.list(None, false).await.unwrap();
        assert_eq!(skills.len(), 2);
    }

    // S-SK-11: test_list_skills_by_scope
    #[tokio::test]
    async fn test_list_skills_by_scope() {
        use crate::services::SkillService;
        let repo = MockSkillRepository::new();
        repo.skills.lock().unwrap().push(Skill::new("global-skill", SkillSource::Inline, SkillScope::Global));
        repo.skills.lock().unwrap().push(Skill::new("project-skill", SkillSource::Inline, SkillScope::project("/my/project")));

        let service = super::SkillServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let global_skills = service.list(Some(SkillScope::Global), false).await.unwrap();
        assert_eq!(global_skills.len(), 1);
        assert_eq!(global_skills[0].name, "global-skill");
    }

    // S-SK-12: test_list_skills_enabled_only
    #[tokio::test]
    async fn test_list_skills_enabled_only() {
        use crate::services::SkillService;
        let repo = MockSkillRepository::new();
        let mut enabled_skill = Skill::new("enabled", SkillSource::Inline, SkillScope::Global);
        enabled_skill.enabled = true;
        let mut disabled_skill = Skill::new("disabled", SkillSource::Inline, SkillScope::Global);
        disabled_skill.enabled = false;

        repo.skills.lock().unwrap().push(enabled_skill);
        repo.skills.lock().unwrap().push(disabled_skill);

        let service = super::SkillServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let enabled_skills = service.list(None, true).await.unwrap();
        assert_eq!(enabled_skills.len(), 1);
        assert_eq!(enabled_skills[0].name, "enabled");
    }

    #[test]
    fn test_validate_content_empty() {
        let service = create_test_service();
        let result = service.validate_content("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_content_valid() {
        let service = create_test_service();
        let result = service.validate_content("# Valid skill\n\nSome instructions.");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_skills() {
        use crate::services::SkillService;
        let repo = MockSkillRepository::new();
        let mut skill = Skill::new("typescript-best", SkillSource::Inline, SkillScope::Global);
        skill.description = Some("TypeScript best practices".to_string());
        repo.skills.lock().unwrap().push(skill);

        let service = super::SkillServiceImpl::new(
            Arc::new(repo),
            Arc::new(MockSkillStorage::new()),
            Arc::new(MockGitHubClient::new()),
            Arc::new(MockUrlClient::new()),
            Arc::new(MockMergeService),
            Arc::new(RwLock::new(EventBus::new())),
        );

        let results = service.search("typescript").await.unwrap();
        assert_eq!(results.len(), 1);
    }
}
