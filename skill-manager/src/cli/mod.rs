//! CLI module for Claude Skill Manager

pub mod commands;

use clap::{Parser, Subcommand};

/// Claude Skill Manager - Manage Claude AI skills
#[derive(Parser, Debug)]
#[command(name = "csm")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Output in JSON format
    #[arg(long, global = true)]
    pub json: bool,

    /// Auto-confirm all prompts
    #[arg(short, long, global = true)]
    pub yes: bool,

    /// Use custom config file
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize CSM configuration
    Init {
        /// Initialize global configuration only
        #[arg(long)]
        global: bool,

        /// Initialize local project only
        #[arg(long)]
        local: bool,

        /// Overwrite existing configuration
        #[arg(long)]
        force: bool,

        /// Import existing CLAUDE.md files
        #[arg(long)]
        import_existing: bool,
    },

    /// Add a skill from various sources
    Add {
        /// Skill source (github:owner/repo, path, URL)
        source: String,

        /// Custom name for the skill
        #[arg(short, long)]
        name: Option<String>,

        /// Scope: global or local
        #[arg(short, long, default_value = "local")]
        scope: String,

        /// Update mode: auto, notify, manual
        #[arg(long, default_value = "auto")]
        update_mode: String,
    },

    /// Remove a skill
    #[command(alias = "rm")]
    Remove {
        /// Skill name
        skill: String,

        /// Remove without confirmation
        #[arg(long)]
        force: bool,
    },

    /// List skills
    #[command(alias = "ls")]
    List {
        /// Filter by scope: all, global, local
        #[arg(short, long, default_value = "all")]
        scope: String,

        /// Show only enabled skills
        #[arg(long)]
        enabled: bool,

        /// Show only disabled skills
        #[arg(long)]
        disabled: bool,
    },

    /// Show skill details
    Show {
        /// Skill name
        skill: String,

        /// Show full content
        #[arg(long)]
        content: bool,
    },

    /// Enable a skill
    Enable {
        /// Skill name
        skill: String,
    },

    /// Disable a skill
    Disable {
        /// Skill name
        skill: String,
    },

    /// Update skills from their sources
    #[command(alias = "up")]
    Update {
        /// Specific skill to update (updates all if omitted)
        skill: Option<String>,

        /// Check for updates without applying
        #[arg(long)]
        check: bool,

        /// Show what would be updated
        #[arg(long)]
        dry_run: bool,
    },

    /// Detect and resolve skill conflicts
    Conflicts {
        /// Interactive resolution
        #[arg(long)]
        resolve: bool,
    },

    /// Search for skills
    #[command(alias = "s")]
    Search {
        /// Search query
        query: String,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Synchronize skill state
    Sync {
        /// Force rebuild of merged CLAUDE.md files
        #[arg(long)]
        rebuild: bool,

        /// Verify symlink integrity
        #[arg(long)]
        verify: bool,
    },

    /// Export skills
    Export {
        /// Export all skills
        #[arg(long)]
        all: bool,

        /// Export specific skill
        #[arg(long)]
        skill: Option<String>,

        /// Output format: json, toml
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file (stdout if omitted)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Import skills
    Import {
        /// Source file or URL
        source: String,

        /// Merge with existing skills
        #[arg(long)]
        merge: bool,

        /// Preview import
        #[arg(long)]
        dry_run: bool,
    },

    /// Create a new skill
    Create {
        /// Skill name
        name: String,

        /// Create from existing file
        #[arg(long)]
        from: Option<String>,

        /// Scope: global or local
        #[arg(short, long, default_value = "local")]
        scope: String,

        /// Open in editor after creation
        #[arg(long)]
        edit: bool,
    },

    /// Edit a skill
    Edit {
        /// Skill name
        skill: String,

        /// Use specific editor
        #[arg(long)]
        editor: Option<String>,
    },

    /// Launch TUI interface
    Ui {
        /// Start in specific section
        #[arg(long)]
        section: Option<String>,
    },

    /// Diagnose and repair issues
    Doctor {
        /// Attempt to fix detected issues
        #[arg(long)]
        fix: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell: bash, zsh, fish, powershell
        shell: String,
    },

    /// Migrate from legacy ~/.csm to XDG-compliant ~/.config/csm
    Migrate {
        /// Show what would be migrated without making changes
        #[arg(long)]
        dry_run: bool,

        /// Overwrite existing destination directory
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
    },

    /// List all configuration
    List,

    /// Open config in editor
    Edit,

    /// Reset to defaults
    Reset {
        /// Reset without confirmation
        #[arg(long)]
        force: bool,
    },
}

impl Cli {
    /// Parse command line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
