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

    // D-EV-01: test_event_bus_subscribe_publish
    #[test]
    fn test_event_bus_subscribe_publish() {
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

        assert_eq!(count.load(Ordering::SeqCst), 0);
        bus.publish(&event);
        assert_eq!(count.load(Ordering::SeqCst), 1);
        bus.publish(&event);
        assert_eq!(count.load(Ordering::SeqCst), 2);
    }

    // D-EV-02: test_event_bus_multiple_subscribers
    #[test]
    fn test_event_bus_multiple_subscribers() {
        let mut bus = EventBus::new();
        let count1 = Arc::new(AtomicUsize::new(0));
        let count2 = Arc::new(AtomicUsize::new(0));
        let count3 = Arc::new(AtomicUsize::new(0));

        bus.subscribe(Box::new(CountingHandler { count: count1.clone() }));
        bus.subscribe(Box::new(CountingHandler { count: count2.clone() }));
        bus.subscribe(Box::new(CountingHandler { count: count3.clone() }));

        let event = DomainEvent::skill_added(
            Uuid::new_v4(),
            "test",
            SkillSource::Inline,
            SkillScope::Global,
        );

        bus.publish(&event);

        // All three handlers should receive the event
        assert_eq!(count1.load(Ordering::SeqCst), 1);
        assert_eq!(count2.load(Ordering::SeqCst), 1);
        assert_eq!(count3.load(Ordering::SeqCst), 1);
    }

    // D-EV-03: test_domain_event_variants
    #[test]
    fn test_domain_event_variants() {
        let skill_id = Uuid::new_v4();
        let conflict_id = Uuid::new_v4();

        // SkillAdded
        let event = DomainEvent::skill_added(
            skill_id,
            "test-skill",
            SkillSource::Inline,
            SkillScope::Global,
        );
        assert!(event.summary().contains("Added skill"));
        assert!(event.timestamp() <= Utc::now());

        // SkillRemoved
        let event = DomainEvent::skill_removed(skill_id, "test-skill");
        assert!(event.summary().contains("Removed skill"));

        // SkillEnabled
        let event = DomainEvent::skill_enabled(skill_id, "test-skill");
        assert!(event.summary().contains("Enabled skill"));

        // SkillDisabled
        let event = DomainEvent::skill_disabled(skill_id, "test-skill");
        assert!(event.summary().contains("Disabled skill"));

        // SkillUpdated
        let event = DomainEvent::skill_updated(skill_id, "test-skill", "old_hash", "new_hash");
        assert!(event.summary().contains("Updated skill"));

        // ConflictDetected
        let event = DomainEvent::ConflictDetected {
            conflict_id,
            skill_a_id: skill_id,
            skill_b_id: Uuid::new_v4(),
            conflict_type: ConflictType::Duplicate,
            timestamp: Utc::now(),
        };
        assert!(event.summary().contains("Conflict detected"));

        // ConflictResolved
        let event = DomainEvent::ConflictResolved {
            conflict_id,
            resolution: "Kept skill A".to_string(),
            timestamp: Utc::now(),
        };
        assert!(event.summary().contains("Conflict resolved"));

        // SkillsMerged
        let event = DomainEvent::SkillsMerged {
            skill_count: 5,
            output_path: "/path/to/CLAUDE.md".to_string(),
            timestamp: Utc::now(),
        };
        assert!(event.summary().contains("Merged 5 skills"));

        // SystemInitialized
        let event = DomainEvent::SystemInitialized {
            csm_home: "/home/user/.csm".to_string(),
            timestamp: Utc::now(),
        };
        assert!(event.summary().contains("System initialized"));

        // ConfigChanged
        let event = DomainEvent::ConfigChanged {
            key: "auto_update".to_string(),
            old_value: Some("true".to_string()),
            new_value: "false".to_string(),
            timestamp: Utc::now(),
        };
        assert!(event.summary().contains("Config changed"));
    }

    #[test]
    fn test_event_timestamp() {
        let before = Utc::now();
        let event = DomainEvent::skill_added(
            Uuid::new_v4(),
            "test",
            SkillSource::Inline,
            SkillScope::Global,
        );
        let after = Utc::now();

        let ts = event.timestamp();
        assert!(ts >= before);
        assert!(ts <= after);
    }

    #[test]
    fn test_event_summary() {
        let skill_id = Uuid::new_v4();

        assert_eq!(
            DomainEvent::skill_added(skill_id, "my-skill", SkillSource::Inline, SkillScope::Global)
                .summary(),
            "Added skill: my-skill"
        );

        assert_eq!(
            DomainEvent::skill_removed(skill_id, "my-skill").summary(),
            "Removed skill: my-skill"
        );

        assert_eq!(
            DomainEvent::skill_enabled(skill_id, "my-skill").summary(),
            "Enabled skill: my-skill"
        );

        assert_eq!(
            DomainEvent::skill_disabled(skill_id, "my-skill").summary(),
            "Disabled skill: my-skill"
        );

        assert_eq!(
            DomainEvent::skill_updated(skill_id, "my-skill", "old", "new").summary(),
            "Updated skill: my-skill"
        );
    }

    #[test]
    fn test_event_serialization() {
        let event = DomainEvent::skill_added(
            Uuid::new_v4(),
            "test-skill",
            SkillSource::Inline,
            SkillScope::Global,
        );

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("skill_added"));
        assert!(json.contains("test-skill"));

        let deserialized: DomainEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event.summary(), deserialized.summary());
    }

    #[test]
    fn test_event_bus_no_subscribers() {
        let bus = EventBus::new();
        let event = DomainEvent::skill_added(
            Uuid::new_v4(),
            "test",
            SkillSource::Inline,
            SkillScope::Global,
        );

        // Should not panic with no subscribers
        bus.publish(&event);
    }

    #[test]
    fn test_event_bus_default() {
        let bus = EventBus::default();
        let event = DomainEvent::skill_added(
            Uuid::new_v4(),
            "test",
            SkillSource::Inline,
            SkillScope::Global,
        );

        // Should work with default
        bus.publish(&event);
    }
}
