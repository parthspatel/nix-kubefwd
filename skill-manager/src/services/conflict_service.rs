//! Conflict detection and resolution service implementation

use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    Conflict, ConflictType, DomainEvent, EventBus, ResolutionStrategy, Skill,
};
use crate::utils::error::{Error, Result};

use super::traits::{
    ConflictRepository, ConflictService as ConflictServiceTrait, MergeService,
    SkillRepository, SkillStorage,
};

/// Implementation of the conflict detection and resolution service
pub struct ConflictServiceImpl<CR, SR, S, M>
where
    CR: ConflictRepository,
    SR: SkillRepository,
    S: SkillStorage,
    M: MergeService,
{
    conflict_repo: Arc<CR>,
    skill_repo: Arc<SR>,
    storage: Arc<S>,
    merge_service: Arc<M>,
    event_bus: Arc<std::sync::RwLock<EventBus>>,
}

impl<CR, SR, S, M> ConflictServiceImpl<CR, SR, S, M>
where
    CR: ConflictRepository,
    SR: SkillRepository,
    S: SkillStorage,
    M: MergeService,
{
    /// Create a new conflict service
    pub fn new(
        conflict_repo: Arc<CR>,
        skill_repo: Arc<SR>,
        storage: Arc<S>,
        merge_service: Arc<M>,
        event_bus: Arc<std::sync::RwLock<EventBus>>,
    ) -> Self {
        Self {
            conflict_repo,
            skill_repo,
            storage,
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
}

#[async_trait]
impl<CR, SR, S, M> ConflictServiceTrait for ConflictServiceImpl<CR, SR, S, M>
where
    CR: ConflictRepository + 'static,
    SR: SkillRepository + 'static,
    S: SkillStorage + 'static,
    M: MergeService + 'static,
{
    async fn detect(&self) -> Result<Vec<Conflict>> {
        // Get all enabled skills
        let skills = self.skill_repo.list_enabled().await?;

        if skills.len() < 2 {
            return Ok(Vec::new());
        }

        // Load content for each skill
        let mut skill_contents: Vec<(Skill, String)> = Vec::new();
        for skill in skills {
            if let Ok(content) = self.storage.read(skill.id).await {
                skill_contents.push((skill, content));
            }
        }

        // Detect conflicts
        let conflicts = detect_conflicts_internal(&skill_contents);

        // Store detected conflicts
        for conflict in &conflicts {
            self.conflict_repo.create(conflict).await?;

            self.publish_event(DomainEvent::ConflictDetected {
                conflict_id: conflict.id,
                skill_a_id: conflict.skill_a_id,
                skill_b_id: conflict.skill_b_id,
                conflict_type: conflict.conflict_type,
                timestamp: chrono::Utc::now(),
            });
        }

        Ok(conflicts)
    }

    async fn list_unresolved(&self) -> Result<Vec<Conflict>> {
        self.conflict_repo.list_unresolved().await
    }

    async fn resolve(
        &self,
        conflict_id: Uuid,
        strategy: ResolutionStrategy,
    ) -> Result<()> {
        let mut conflict = self
            .conflict_repo
            .get(conflict_id)
            .await?
            .ok_or_else(|| Error::ConflictNotFound(conflict_id.to_string()))?;

        // Apply the resolution strategy
        match strategy {
            ResolutionStrategy::DisableSkillA => {
                if let Some(mut skill) = self.skill_repo.get(conflict.skill_a_id).await? {
                    skill.enabled = false;
                    self.skill_repo.update(&skill).await?;
                }
            }
            ResolutionStrategy::DisableSkillB => {
                if let Some(mut skill) = self.skill_repo.get(conflict.skill_b_id).await? {
                    skill.enabled = false;
                    self.skill_repo.update(&skill).await?;
                }
            }
            ResolutionStrategy::PrioritizeA => {
                // Increase A's priority, decrease B's
                if let Some(mut skill_a) = self.skill_repo.get(conflict.skill_a_id).await? {
                    if let Some(mut skill_b) = self.skill_repo.get(conflict.skill_b_id).await? {
                        skill_a.priority = skill_b.priority + 10;
                        self.skill_repo.update(&skill_a).await?;
                    }
                }
            }
            ResolutionStrategy::PrioritizeB => {
                if let Some(mut skill_a) = self.skill_repo.get(conflict.skill_a_id).await? {
                    if let Some(mut skill_b) = self.skill_repo.get(conflict.skill_b_id).await? {
                        skill_b.priority = skill_a.priority + 10;
                        self.skill_repo.update(&skill_b).await?;
                    }
                }
            }
            ResolutionStrategy::Ignore => {
                // Just mark as ignored, don't change skills
            }
        }

        // Mark conflict as resolved
        conflict.resolve();
        self.conflict_repo.update(&conflict).await?;

        self.publish_event(DomainEvent::ConflictResolved {
            conflict_id,
            resolution: format!("{}", strategy),
            timestamp: chrono::Utc::now(),
        });

        // Rebuild merged output
        self.merge_service.rebuild_all().await?;

        Ok(())
    }

    async fn ignore(&self, conflict_id: Uuid) -> Result<()> {
        self.resolve(conflict_id, ResolutionStrategy::Ignore).await
    }
}

/// Internal function to detect conflicts between skills
fn detect_conflicts_internal(skills: &[(Skill, String)]) -> Vec<Conflict> {
    let mut conflicts = Vec::new();

    // Compare each pair of skills
    for i in 0..skills.len() {
        for j in (i + 1)..skills.len() {
            let (skill_a, content_a) = &skills[i];
            let (skill_b, content_b) = &skills[j];

            // Find duplicates
            conflicts.extend(find_duplicates(skill_a, content_a, skill_b, content_b));

            // Find contradictions
            conflicts.extend(find_contradictions(skill_a, content_a, skill_b, content_b));
        }
    }

    conflicts
}

/// Find duplicate instructions between two skills
fn find_duplicates(
    skill_a: &Skill,
    content_a: &str,
    skill_b: &Skill,
    content_b: &str,
) -> Vec<Conflict> {
    let mut conflicts = Vec::new();

    let instructions_a = extract_instructions(content_a);
    let instructions_b = extract_instructions(content_b);

    for (line_a, inst_a) in &instructions_a {
        for (line_b, inst_b) in &instructions_b {
            let normalized_a = normalize_instruction(inst_a);
            let normalized_b = normalize_instruction(inst_b);

            if normalized_a == normalized_b && !normalized_a.is_empty() {
                conflicts.push(
                    Conflict::builder(skill_a.id, skill_b.id, ConflictType::Duplicate)
                        .description("Duplicate instruction found")
                        .lines(*line_a, *line_b)
                        .content(inst_a, inst_b)
                        .suggestion("Remove from one skill or merge them")
                        .build(),
                );
            }
        }
    }

    conflicts
}

/// Find contradictory instructions between two skills
fn find_contradictions(
    skill_a: &Skill,
    content_a: &str,
    skill_b: &Skill,
    content_b: &str,
) -> Vec<Conflict> {
    let mut conflicts = Vec::new();

    // Keywords that often indicate contradictions
    let contradiction_pairs = [
        ("always", "never"),
        ("must", "must not"),
        ("should", "should not"),
        ("required", "optional"),
        ("enable", "disable"),
        ("use", "avoid"),
        ("prefer", "avoid"),
        ("do", "don't"),
        ("do", "do not"),
    ];

    let instructions_a = extract_instructions(content_a);
    let instructions_b = extract_instructions(content_b);

    for (line_a, inst_a) in &instructions_a {
        let lower_a = inst_a.to_lowercase();

        for (line_b, inst_b) in &instructions_b {
            let lower_b = inst_b.to_lowercase();

            for (word_a, word_b) in &contradiction_pairs {
                // Check if A has word_a and B has word_b (or vice versa)
                let has_contradiction = (lower_a.contains(word_a) && lower_b.contains(word_b))
                    || (lower_a.contains(word_b) && lower_b.contains(word_a));

                if has_contradiction && same_topic(&lower_a, &lower_b) {
                    conflicts.push(
                        Conflict::builder(skill_a.id, skill_b.id, ConflictType::Contradictory)
                            .description(format!(
                                "Contradictory instructions: '{}' vs '{}'",
                                word_a, word_b
                            ))
                            .lines(*line_a, *line_b)
                            .content(inst_a, inst_b)
                            .suggestion("Disable one skill or set priority")
                            .build(),
                    );
                    break; // Only report one contradiction per pair
                }
            }
        }
    }

    conflicts
}

/// Extract instructions (list items) from content
fn extract_instructions(content: &str) -> Vec<(usize, String)> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let trimmed = line.trim();
            trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with("•")
        })
        .map(|(i, line)| (i + 1, line.trim().to_string()))
        .collect()
}

/// Normalize an instruction for comparison
fn normalize_instruction(inst: &str) -> String {
    inst.trim()
        .trim_start_matches('-')
        .trim_start_matches('*')
        .trim_start_matches('•')
        .trim()
        .to_lowercase()
}

/// Check if two instructions are about the same topic
fn same_topic(a: &str, b: &str) -> bool {
    let words_a: HashSet<_> = a
        .split_whitespace()
        .filter(|w| w.len() > 3)
        .filter(|w| !is_common_word(w))
        .collect();

    let words_b: HashSet<_> = b
        .split_whitespace()
        .filter(|w| w.len() > 3)
        .filter(|w| !is_common_word(w))
        .collect();

    if words_a.is_empty() || words_b.is_empty() {
        return false;
    }

    let intersection: HashSet<_> = words_a.intersection(&words_b).collect();
    let min_len = words_a.len().min(words_b.len());

    // At least 30% word overlap
    (intersection.len() as f64 / min_len as f64) > 0.3
}

/// Check if a word is a common/stop word
fn is_common_word(word: &str) -> bool {
    const COMMON_WORDS: &[&str] = &[
        "the", "and", "for", "with", "that", "this", "from", "your", "when",
        "always", "never", "should", "must", "will", "have", "been", "being",
        "use", "using", "used",
    ];
    COMMON_WORDS.contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{SkillScope, SkillSource};

    fn make_skill(name: &str) -> Skill {
        Skill::new(name, SkillSource::Inline, SkillScope::Global)
    }

    #[test]
    fn test_find_duplicates() {
        let skill_a = make_skill("skill-a");
        let skill_b = make_skill("skill-b");

        let content_a = "# Style\n\n- Use 2-space indentation\n- Be consistent";
        let content_b = "# Format\n\n- Use 2-space indentation\n- Write tests";

        let conflicts = find_duplicates(&skill_a, content_a, &skill_b, content_b);

        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflict_type, ConflictType::Duplicate);
    }

    #[test]
    fn test_find_contradictions() {
        let skill_a = make_skill("skill-a");
        let skill_b = make_skill("skill-b");

        let content_a = "# Style\n\n- Always use strict null checks";
        let content_b = "# Style\n\n- Never use strict null checks";

        let conflicts = find_contradictions(&skill_a, content_a, &skill_b, content_b);

        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflict_type, ConflictType::Contradictory);
    }

    #[test]
    fn test_no_conflict_different_topics() {
        let skill_a = make_skill("skill-a");
        let skill_b = make_skill("skill-b");

        let content_a = "# TypeScript\n\n- Always use strict mode";
        let content_b = "# Python\n\n- Never use global variables";

        let conflicts = find_contradictions(&skill_a, content_a, &skill_b, content_b);

        // Different topics, so no contradiction
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_extract_instructions() {
        let content = "# Header\n\n- Item 1\n* Item 2\nNot an item\n- Item 3";
        let instructions = extract_instructions(content);

        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0].1, "- Item 1");
        assert_eq!(instructions[1].1, "* Item 2");
        assert_eq!(instructions[2].1, "- Item 3");
    }

    #[test]
    fn test_normalize_instruction() {
        assert_eq!(normalize_instruction("- Use tabs"), "use tabs");
        assert_eq!(normalize_instruction("* Use tabs"), "use tabs");
        assert_eq!(normalize_instruction("  - Use tabs  "), "use tabs");
    }

    #[test]
    fn test_same_topic() {
        assert!(same_topic(
            "use strict typescript configuration",
            "enable strict typescript mode"
        ));
        assert!(!same_topic(
            "use python type hints",
            "enable javascript linting"
        ));
    }
}
