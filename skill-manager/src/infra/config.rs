//! Configuration management

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::services::ConfigManager;
use crate::utils::error::{Error, Result};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub updates: UpdateConfig,

    #[serde(default)]
    pub github: GitHubConfig,

    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_scope")]
    pub default_scope: String,

    #[serde(default)]
    pub editor: Option<String>,

    #[serde(default = "default_true")]
    pub color: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    #[serde(default = "default_update_mode")]
    pub mode: String,

    #[serde(default = "default_schedule")]
    pub schedule: String,

    #[serde(default = "default_true")]
    pub check_on_startup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    #[serde(default = "default_ref")]
    pub default_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_theme")]
    pub theme: String,

    #[serde(default = "default_true")]
    pub show_welcome: bool,
}

fn default_scope() -> String {
    "local".to_string()
}
fn default_true() -> bool {
    true
}
fn default_update_mode() -> String {
    "auto".to_string()
}
fn default_schedule() -> String {
    "daily".to_string()
}
fn default_ref() -> String {
    "main".to_string()
}
fn default_theme() -> String {
    "dark".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            updates: UpdateConfig::default(),
            github: GitHubConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            default_scope: default_scope(),
            editor: None,
            color: default_true(),
        }
    }
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            mode: default_update_mode(),
            schedule: default_schedule(),
            check_on_startup: default_true(),
        }
    }
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            default_ref: default_ref(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            show_welcome: default_true(),
        }
    }
}

/// Configuration manager implementation
pub struct ConfigManagerImpl {
    csm_home: PathBuf,
    config: Config,
}

impl ConfigManagerImpl {
    /// Create a new config manager
    pub fn new(csm_home: PathBuf) -> Self {
        Self {
            csm_home,
            config: Config::default(),
        }
    }

    /// Load configuration from disk
    pub fn load(&mut self) -> Result<()> {
        let config_path = self.config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| Error::Config(format!("Failed to read config: {}", e)))?;

            self.config = toml::from_str(&content)?;
        }

        Ok(())
    }

    /// Save configuration to disk
    pub fn save(&self) -> Result<()> {
        let config_path = self.config_path();

        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| Error::Config(format!("Failed to create config dir: {}", e)))?;
        }

        let content = toml::to_string_pretty(&self.config)?;
        std::fs::write(&config_path, content)
            .map_err(|e| Error::Config(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    /// Get the config file path
    fn config_path(&self) -> PathBuf {
        self.csm_home.join("config.toml")
    }

    /// Get a reference to the config
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get a mutable reference to the config
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Detect CSM home directory.
    ///
    /// Priority:
    /// 1. `CSM_HOME` environment variable (explicit override)
    /// 2. `XDG_CONFIG_HOME/csm` if XDG_CONFIG_HOME is set
    /// 3. `~/.config/csm` (XDG default)
    pub fn detect_csm_home() -> PathBuf {
        // 1. Check CSM_HOME environment variable (explicit override)
        if let Ok(path) = std::env::var("CSM_HOME") {
            return PathBuf::from(path);
        }

        // 2. Check XDG_CONFIG_HOME environment variable
        if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg_config).join("csm");
        }

        // 3. Default to ~/.config/csm
        if let Some(base_dirs) = directories::BaseDirs::new() {
            return base_dirs.home_dir().join(".config").join("csm");
        }

        // 4. Fallback
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("csm")
    }

    /// Detect legacy CSM home directory (~/.csm) if it exists.
    ///
    /// Returns `Some(path)` if `~/.csm` exists, `None` otherwise.
    pub fn detect_legacy_home() -> Option<PathBuf> {
        directories::BaseDirs::new()
            .map(|dirs| dirs.home_dir().join(".csm"))
            .filter(|p| p.exists())
    }

    /// Check if migration from legacy ~/.csm is needed.
    ///
    /// Returns `true` if:
    /// - Legacy `~/.csm` exists
    /// - New XDG location doesn't exist yet
    /// - `CSM_HOME` is not set (user hasn't explicitly chosen a location)
    pub fn needs_migration() -> bool {
        if std::env::var("CSM_HOME").is_ok() {
            return false;
        }

        let legacy_exists = Self::detect_legacy_home().is_some();
        let new_home = Self::detect_csm_home();
        let new_exists = new_home.exists();

        legacy_exists && !new_exists
    }
}

impl ConfigManager for ConfigManagerImpl {
    fn get(&self, key: &str) -> Option<String> {
        match key {
            "general.default_scope" => Some(self.config.general.default_scope.clone()),
            "general.editor" => self.config.general.editor.clone(),
            "general.color" => Some(self.config.general.color.to_string()),
            "updates.mode" => Some(self.config.updates.mode.clone()),
            "updates.schedule" => Some(self.config.updates.schedule.clone()),
            "updates.check_on_startup" => Some(self.config.updates.check_on_startup.to_string()),
            "github.default_ref" => Some(self.config.github.default_ref.clone()),
            "ui.theme" => Some(self.config.ui.theme.clone()),
            "ui.show_welcome" => Some(self.config.ui.show_welcome.to_string()),
            _ => None,
        }
    }

    fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "general.default_scope" => self.config.general.default_scope = value.to_string(),
            "general.editor" => self.config.general.editor = Some(value.to_string()),
            "general.color" => {
                self.config.general.color = value
                    .parse()
                    .map_err(|_| Error::Config(format!("Invalid boolean value: {}", value)))?;
            }
            "updates.mode" => self.config.updates.mode = value.to_string(),
            "updates.schedule" => self.config.updates.schedule = value.to_string(),
            "updates.check_on_startup" => {
                self.config.updates.check_on_startup = value
                    .parse()
                    .map_err(|_| Error::Config(format!("Invalid boolean value: {}", value)))?;
            }
            "github.default_ref" => self.config.github.default_ref = value.to_string(),
            "ui.theme" => self.config.ui.theme = value.to_string(),
            "ui.show_welcome" => {
                self.config.ui.show_welcome = value
                    .parse()
                    .map_err(|_| Error::Config(format!("Invalid boolean value: {}", value)))?;
            }
            _ => return Err(Error::Config(format!("Unknown config key: {}", key))),
        }
        self.save()
    }

    fn csm_home(&self) -> &Path {
        &self.csm_home
    }

    fn global_skills_dir(&self) -> PathBuf {
        self.csm_home.join("skills")
    }

    fn cache_dir(&self) -> PathBuf {
        self.csm_home.join("cache")
    }

    fn database_path(&self) -> PathBuf {
        self.csm_home.join("registry.db")
    }

    fn is_initialized(&self) -> bool {
        self.csm_home.exists() && self.database_path().exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.default_scope, "local");
        assert!(config.general.color);
        assert_eq!(config.updates.mode, "auto");
    }

    #[test]
    fn test_config_manager_paths() {
        let temp = tempdir().unwrap();
        let manager = ConfigManagerImpl::new(temp.path().to_path_buf());

        assert_eq!(manager.csm_home(), temp.path());
        assert_eq!(manager.global_skills_dir(), temp.path().join("skills"));
        assert_eq!(manager.database_path(), temp.path().join("registry.db"));
    }

    #[test]
    fn test_config_save_load() {
        let temp = tempdir().unwrap();
        let mut manager = ConfigManagerImpl::new(temp.path().to_path_buf());

        manager.config_mut().general.default_scope = "global".to_string();
        manager.save().unwrap();

        let mut manager2 = ConfigManagerImpl::new(temp.path().to_path_buf());
        manager2.load().unwrap();

        assert_eq!(manager2.config().general.default_scope, "global");
    }

    #[test]
    fn test_config_get_set() {
        let temp = tempdir().unwrap();
        let mut manager = ConfigManagerImpl::new(temp.path().to_path_buf());

        manager.set("general.default_scope", "global").unwrap();
        assert_eq!(
            manager.get("general.default_scope"),
            Some("global".to_string())
        );

        manager.set("general.color", "false").unwrap();
        assert_eq!(manager.get("general.color"), Some("false".to_string()));
    }
}
