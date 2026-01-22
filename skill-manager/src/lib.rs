//! Claude Skill Manager Library
//!
//! This crate provides the core functionality for managing Claude AI skills,
//! including adding, removing, updating, and merging skills from various sources.
//!
//! # Architecture
//!
//! The library is organized into the following modules:
//!
//! - `domain`: Core domain models (Skill, Source, Conflict, etc.)
//! - `services`: Application services with business logic
//! - `infra`: Infrastructure implementations (database, storage, API clients)
//! - `cli`: Command-line interface
//! - `tui`: Terminal user interface
//! - `utils`: Utility functions and error handling
//!
//! # Example
//!
//! ```rust,ignore
//! use csm::domain::{Skill, SkillScope, SkillSource};
//! use csm::services::SkillService;
//!
//! // Add a skill from GitHub
//! let skill = service.add(
//!     "github:anthropics/claude-skills/typescript",
//!     None,
//!     SkillScope::Global,
//! ).await?;
//! ```

pub mod domain;
pub mod services;
pub mod infra;
pub mod cli;
pub mod tui;
pub mod utils;

// Re-export commonly used types
pub use domain::{Skill, SkillScope, SkillSource, Conflict, ConflictType};
pub use utils::error::{Error, Result};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const APP_NAME: &str = "csm";

/// Full application name
pub const APP_FULL_NAME: &str = "Claude Skill Manager";
