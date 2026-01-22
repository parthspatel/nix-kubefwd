//! Domain events for skill management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ConflictType, SkillScope, SkillSource};

/// Domain events that occur during skill management
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DomainEvent {
    /// A skill was added to the registry
    SkillAdded {
        skill_id: Uuid,
        name: String,
        source: SkillSource,
        scope: SkillScope,
        timestamp: DateTime<Utc>,
    },

    /// A skill was removed from the registry
    SkillRemoved {
        skill_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },

    /// A skill was enabled
    SkillEnabled {
        skill_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },

    /// A skill was disabled
    SkillDisabled {
        skill_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },

    /// A skill was updated from its source
    SkillUpdated {
        skill_id: Uuid,
        name: String,
        old_hash: String,
        new_hash: String,
        timestamp: DateTime<Utc>,
    },

    /// A conflict was detected
    ConflictDetected {
        conflict_id: Uuid,
        skill_a_id: Uuid,
        skill_b_id: Uuid,
        conflict_type: ConflictType,
        timestamp: DateTime<Utc>,
    },

    /// A conflict was resolved
    ConflictResolved {
        conflict_id: Uuid,
        resolution: String,
        timestamp: DateTime<Utc>,
    },

    /// Skills were merged into CLAUDE.md
    SkillsMerged {
        skill_count: usize,
        output_path: String,
        timestamp: DateTime<Utc>,
    },

    /// System was initialized
    SystemInitialized {
        csm_home: String,
        timestamp: DateTime<Utc>,
    },

    /// Configuration was changed
    ConfigChanged {
        key: String,
        old_value: Option<String>,
        new_value: String,
        timestamp: DateTime<Utc>,
    },
}

impl DomainEvent {
    /// Get the timestamp of this event
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Self::SkillAdded { timestamp, .. } => *timestamp,
            Self::SkillRemoved { timestamp, .. } => *timestamp,
            Self::SkillEnabled { timestamp, .. } => *timestamp,
            Self::SkillDisabled { timestamp, .. } => *timestamp,
            Self::SkillUpdated { timestamp, .. } => *timestamp,
            Self::ConflictDetected { timestamp, .. } => *timestamp,
            Self::ConflictResolved { timestamp, .. } => *timestamp,
            Self::SkillsMerged { timestamp, .. } => *timestamp,
            Self::SystemInitialized { timestamp, .. } => *timestamp,
            Self::ConfigChanged { timestamp, .. } => *timestamp,
        }
    }

    /// Get a short description of this event
    pub fn summary(&self) -> String {
        match self {
            Self::SkillAdded { name, .. } => format!("Added skill: {}", name),
            Self::SkillRemoved { name, .. } => format!("Removed skill: {}", name),
            Self::SkillEnabled { name, .. } => format!("Enabled skill: {}", name),
            Self::SkillDisabled { name, .. } => format!("Disabled skill: {}", name),
            Self::SkillUpdated { name, .. } => format!("Updated skill: {}", name),
            Self::ConflictDetected { conflict_type, .. } => {
                format!("Conflict detected: {}", conflict_type)
            }
            Self::ConflictResolved { resolution, .. } => {
                format!("Conflict resolved: {}", resolution)
            }
            Self::SkillsMerged { skill_count, .. } => {
                format!("Merged {} skills", skill_count)
            }
            Self::SystemInitialized { .. } => "System initialized".to_string(),
            Self::ConfigChanged { key, .. } => format!("Config changed: {}", key),
        }
    }

    /// Create a SkillAdded event
    pub fn skill_added(
        skill_id: Uuid,
        name: impl Into<String>,
        source: SkillSource,
        scope: SkillScope,
    ) -> Self {
        Self::SkillAdded {
            skill_id,
            name: name.into(),
            source,
            scope,
            timestamp: Utc::now(),
        }
    }

    /// Create a SkillRemoved event
    pub fn skill_removed(skill_id: Uuid, name: impl Into<String>) -> Self {
        Self::SkillRemoved {
            skill_id,
            name: name.into(),
            timestamp: Utc::now(),
        }
    }

    /// Create a SkillEnabled event
    pub fn skill_enabled(skill_id: Uuid, name: impl Into<String>) -> Self {
        Self::SkillEnabled {
            skill_id,
            name: name.into(),
            timestamp: Utc::now(),
        }
    }

    /// Create a SkillDisabled event
    pub fn skill_disabled(skill_id: Uuid, name: impl Into<String>) -> Self {
        Self::SkillDisabled {
            skill_id,
            name: name.into(),
            timestamp: Utc::now(),
        }
    }

    /// Create a SkillUpdated event
    pub fn skill_updated(
        skill_id: Uuid,
        name: impl Into<String>,
        old_hash: impl Into<String>,
        new_hash: impl Into<String>,
    ) -> Self {
        Self::SkillUpdated {
            skill_id,
            name: name.into(),
            old_hash: old_hash.into(),
            new_hash: new_hash.into(),
            timestamp: Utc::now(),
        }
    }
}

/// Event handler trait for processing domain events
pub trait EventHandler: Send + Sync {
    /// Handle a domain event
    fn handle(&self, event: &DomainEvent);
}

/// Simple event bus for publishing and subscribing to events
#[derive(Default)]
pub struct EventBus {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self::default()
    }

    /// Subscribe a handler to the event bus
    pub fn subscribe(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    /// Publish an event to all handlers
    pub fn publish(&self, event: &DomainEvent) {
        for handler in &self.handlers {
            handler.handle(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct CountingHandler {
        count: Arc<AtomicUsize>,
    }

    impl EventHandler for CountingHandler {
        fn handle(&self, _event: &DomainEvent) {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_event_bus() {
        let mut bus = EventBus::new();
        let count = Arc::new(AtomicUsize::new(0));

        bus.subscribe(Box::new(CountingHandler {
            count: count.clone(),
        }));

        let event = DomainEvent::skill_added(
            Uuid::new_v4(),
            "test",
            SkillSource::Inline,
            SkillScope::Global,
        );

        bus.publish(&event);
        bus.publish(&event);

        assert_eq!(count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_event_summary() {
        let event = DomainEvent::skill_added(
            Uuid::new_v4(),
            "my-skill",
            SkillSource::Inline,
            SkillScope::Global,
        );

        assert_eq!(event.summary(), "Added skill: my-skill");
    }
}
