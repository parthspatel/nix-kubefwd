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
    use super::*;

    // Tests will be added in the test planning phase
}
